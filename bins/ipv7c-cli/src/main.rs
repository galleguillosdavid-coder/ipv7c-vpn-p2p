//! IPv7C — Sovereign P2P VPN CLI

use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "ipv7c", version, about = "Sovereign P2P VPN — Zero-config, Post-Quantum, Self-Governing")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the VPN node
    Up,
    /// Show node status
    Status,
    /// List connected peers
    Peers,
    /// Manage network profiles
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    /// Manage node identity
    Identity {
        #[command(subcommand)]
        action: IdentityAction,
    },
}

#[derive(Subcommand)]
enum ProfileAction {
    /// List all profiles
    List,
    /// Create a new profile
    Create { name: String },
    /// Switch to a profile
    Use { name: String },
    /// Delete a profile
    Delete { name: String },
}

#[derive(Subcommand)]
enum IdentityAction {
    /// Show current identity (DID and alias)
    Show,
    /// Generate a new identity
    Generate,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();
    let config = ipv7c_config::NodeConfig::load_or_default();

    match cli.command {
        Commands::Up => {
            println!("{}", "╔══════════════════════════════════════╗".cyan());
            println!("{}", "║   IPv7C — Sovereign Mesh VPN v1.0    ║".cyan());
            println!("{}", "╚══════════════════════════════════════╝".cyan());
            println!();

            let mut node = ipv7c_node::lifecycle::SovereignNode::new(config);
            node.start().await?;
        }
        Commands::Status => {
            println!("{}: IPv7C Node", "Status".green().bold());
            println!("  Port: {}", config.listen_port);
            println!("  PQC: {}", if config.crypto.pqc_enabled { "enabled".green() } else { "disabled".red() });
            println!("  mDNS: {}", config.discovery.mdns_enabled);
            println!("  DHT: {}", config.discovery.dht_enabled);
        }
        Commands::Peers => {
            println!("{}", "Connected peers:".bold());
            println!("  (node not running — start with `ipv7c up`)");
        }
        Commands::Profile { action } => {
            let wallet_path = ipv7c_config::defaults::wallet_path();
            let wallet = ipv7c_identity::wallet::Wallet::open(&wallet_path)?;
            match action {
                ProfileAction::List => {
                    let profiles = wallet.list_profiles()?;
                    let active = wallet.active_profile_name()?;
                    println!("{}", "Profiles:".bold());
                    for name in profiles {
                        let marker = if active.as_deref() == Some(&name) { " ●".green().to_string() } else { "  ".to_string() };
                        let p = wallet.load_profile(&name)?;
                        println!("{marker} {name} — {} ({})", p.alias().yellow(), p.did());
                    }
                }
                ProfileAction::Create { name } => {
                    let p = ipv7c_identity::profile::Profile::new(&name);
                    wallet.save_profile(&p)?;
                    println!("{} Profile '{}' created — {}", "✓".green(), name, p.alias().yellow());
                }
                ProfileAction::Use { name } => {
                    wallet.set_active(&name)?;
                    println!("{} Switched to profile '{}'", "✓".green(), name);
                }
                ProfileAction::Delete { name } => {
                    wallet.delete_profile(&name)?;
                    println!("{} Profile '{}' deleted", "✓".green(), name);
                }
            }
        }
        Commands::Identity { action } => {
            let wallet_path = ipv7c_config::defaults::wallet_path();
            let wallet = ipv7c_identity::wallet::Wallet::open(&wallet_path)?;
            match action {
                IdentityAction::Show => {
                    let p = wallet.ensure_default_profile()?;
                    let did = p.did();
                    println!("{}", "Node Identity".bold());
                    println!("  DID:   {}", did.uri.cyan());
                    println!("  Alias: {}", p.alias().yellow());
                    println!("  Profile: {}", p.name);
                }
                IdentityAction::Generate => {
                    let p = ipv7c_identity::profile::Profile::new("default");
                    wallet.save_profile(&p)?;
                    wallet.set_active("default")?;
                    println!("{} New identity generated: {}", "✓".green(), p.alias().yellow());
                }
            }
        }
    }
    Ok(())
}
