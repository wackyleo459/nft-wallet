use std::{process::Command, env, fs, path::Path};
use sha2::{Sha256, Digest};

use walkdir::WalkDir;

fn main() {
    env::set_current_dir(env::var_os("CARGO_MANIFEST_DIR").unwrap()).unwrap();
    println!("cargo:rerun-if-env-changed=DFX_NETWORK");
    if matches!(env::var("DFX_NETWORK"), Ok(network) if network == "ic") {
        println!("cargo:rustc-cfg=mainnet");
        env::set_var("NODE_ENV", "production");
    }
    println!("cargo:rerun-if-changed=frontend/src");
    println!("cargo:rerun-if-changed=frontend/rollup.config.js");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=frontend/package.json");
    println!("cargo:rerun-if-changed=dfx.json");
    println!("cargo:rerun-if-changed=canister_ids.json");
    let install = Command::new("npm").args(["run", "install"]).status().unwrap();
    assert!(install.success());
    let build = Command::new("npm").args(["run", "build"]).status().unwrap();
    assert!(build.success());
    let hash_dir = Path::new(&env::var_os("OUT_DIR").unwrap()).join("hashes");
    fs::create_dir_all(&hash_dir).unwrap();
    let public_dir = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("frontend/public");
    for entry in WalkDir::new(&public_dir) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }
        let cksum = Sha256::digest(fs::read(entry.path()).unwrap());
        let cksum_file = hash_dir.join(entry.path().strip_prefix(&public_dir).unwrap());
        if let Some(parent) = cksum_file.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(cksum_file, &cksum).unwrap();
    }
}
