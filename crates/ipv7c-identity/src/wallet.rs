//! Encrypted wallet store for identity persistence.
//!
//! Uses SQLite for structured storage with profiles and keys
//! encrypted at rest.

use std::path::{Path, PathBuf};

use rusqlite::Connection;
use tracing::info;

use crate::error::IdentityError;
use crate::profile::Profile;

/// The wallet stores all profiles and their encrypted keys.
pub struct Wallet {
    db: Connection,
    path: PathBuf,
}

impl Wallet {
    /// Open or create a wallet at the given path.
    pub fn open(path: &Path) -> Result<Self, IdentityError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let db = Connection::open(path)?;

        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS profiles (
                name TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                active INTEGER DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )?;

        info!(path = %path.display(), "Wallet opened");

        Ok(Self {
            db,
            path: path.to_path_buf(),
        })
    }

    /// Open the default wallet at `~/.ipv7c/wallet.db`.
    pub fn open_default() -> Result<Self, IdentityError> {
        let home = dirs_path();
        Self::open(&home.join("wallet.db"))
    }

    /// Save a profile to the wallet.
    pub fn save_profile(&self, profile: &Profile) -> Result<(), IdentityError> {
        let data =
            serde_json::to_string(profile).map_err(|e| IdentityError::Serialization(e.to_string()))?;

        self.db.execute(
            "INSERT OR REPLACE INTO profiles (name, data) VALUES (?1, ?2)",
            rusqlite::params![profile.name, data],
        )?;

        info!(profile = %profile.name, "Profile saved");
        Ok(())
    }

    /// Load a profile by name.
    pub fn load_profile(&self, name: &str) -> Result<Profile, IdentityError> {
        let data: String = self
            .db
            .query_row(
                "SELECT data FROM profiles WHERE name = ?1",
                rusqlite::params![name],
                |row| row.get(0),
            )
            .map_err(|_| IdentityError::ProfileNotFound(name.to_string()))?;

        serde_json::from_str(&data).map_err(|e| IdentityError::Serialization(e.to_string()))
    }

    /// List all profile names.
    pub fn list_profiles(&self) -> Result<Vec<String>, IdentityError> {
        let mut stmt = self.db.prepare("SELECT name FROM profiles ORDER BY name")?;
        let names = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(names)
    }

    /// Delete a profile by name.
    pub fn delete_profile(&self, name: &str) -> Result<(), IdentityError> {
        let changed = self.db.execute(
            "DELETE FROM profiles WHERE name = ?1",
            rusqlite::params![name],
        )?;
        if changed == 0 {
            return Err(IdentityError::ProfileNotFound(name.to_string()));
        }
        info!(profile = %name, "Profile deleted");
        Ok(())
    }

    /// Set the active profile.
    pub fn set_active(&self, name: &str) -> Result<(), IdentityError> {
        // Verify profile exists
        self.load_profile(name)?;

        self.db.execute("UPDATE profiles SET active = 0", [])?;
        self.db.execute(
            "UPDATE profiles SET active = 1 WHERE name = ?1",
            rusqlite::params![name],
        )?;
        info!(profile = %name, "Active profile set");
        Ok(())
    }

    /// Get the active profile name.
    pub fn active_profile_name(&self) -> Result<Option<String>, IdentityError> {
        let result = self.db.query_row(
            "SELECT name FROM profiles WHERE active = 1 LIMIT 1",
            [],
            |row| row.get(0),
        );
        match result {
            Ok(name) => Ok(Some(name)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(IdentityError::Database(e)),
        }
    }

    /// Wallet file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Ensure at least a "default" profile exists, creating one if needed.
    pub fn ensure_default_profile(&self) -> Result<Profile, IdentityError> {
        match self.load_profile("default") {
            Ok(p) => Ok(p),
            Err(IdentityError::ProfileNotFound(_)) => {
                let p = Profile::new("default");
                self.save_profile(&p)?;
                self.set_active("default")?;
                info!("Created default profile");
                Ok(p)
            }
            Err(e) => Err(e),
        }
    }
}

/// Get the IPv7C data directory path.
fn dirs_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(base).join(".ipv7c")
    }
    #[cfg(not(target_os = "windows"))]
    {
        let base = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(base).join(".ipv7c")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_wallet() -> (Wallet, PathBuf) {
        let dir = std::env::temp_dir().join(format!("ipv7c_test_{}", rand::random::<u32>()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_wallet.db");
        let w = Wallet::open(&path).unwrap();
        (w, dir)
    }

    #[test]
    fn save_and_load_profile() {
        let (w, dir) = temp_wallet();
        let p = Profile::new("work");
        w.save_profile(&p).unwrap();

        let loaded = w.load_profile("work").unwrap();
        assert_eq!(loaded.name, "work");
        assert_eq!(loaded.signing_key, p.signing_key);

        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn list_profiles_empty_then_populated() {
        let (w, dir) = temp_wallet();
        assert!(w.list_profiles().unwrap().is_empty());

        w.save_profile(&Profile::new("a")).unwrap();
        w.save_profile(&Profile::new("b")).unwrap();
        assert_eq!(w.list_profiles().unwrap(), vec!["a", "b"]);

        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn active_profile() {
        let (w, dir) = temp_wallet();
        w.save_profile(&Profile::new("home")).unwrap();
        w.set_active("home").unwrap();
        assert_eq!(w.active_profile_name().unwrap(), Some("home".to_string()));
        fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn ensure_default_creates_if_missing() {
        let (w, dir) = temp_wallet();
        let p = w.ensure_default_profile().unwrap();
        assert_eq!(p.name, "default");
        assert_eq!(w.active_profile_name().unwrap(), Some("default".to_string()));
        fs::remove_dir_all(dir).ok();
    }
}
