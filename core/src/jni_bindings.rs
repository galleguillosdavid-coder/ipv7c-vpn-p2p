#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

/// Expose get_self_integrity_hash to Android Kotlin VpnService
#[no_mangle]
pub extern "system" fn Java_com_ipv7c_mobile_IPv7cVpnService_getSelfIntegrityHash(
    mut env: JNIEnv,
    _class: JClass,
    file_path: JString,
) -> jstring {
    let path: String = env
        .get_string(&file_path)
        .expect("Couldn't get java string!")
        .into();

    let result = if path.is_empty() {
        "0000000000000000000000000000000000000000000000000000000000000000".to_string()
    } else {
        crate::get_self_integrity_hash_rust(Some(path)).unwrap_or_else(|_| {
            "0000000000000000000000000000000000000000000000000000000000000000".to_string()
        })
    };

    let output = env.new_string(result)
        .expect("Couldn't create java string!");
        
    output.into_raw()
}

/// Start Tokio runtime from Android
#[no_mangle]
pub extern "system" fn Java_com_ipv7c_mobile_IPv7cVpnService_initTokioRuntime(
    _env: JNIEnv,
    _class: JClass,
) {
    // Scaffold para inicializar Tokio en Android
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Main Android loop
        });
    });
}
