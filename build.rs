use std::env;
use std::fs;
use std::path::PathBuf;
use toml_edit::{value, DocumentMut};

fn main() {
    println!("cargo:rerun-if-changed=extension.toml");

    let extension_toml_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("extension.toml");
    let mut extension_toml: DocumentMut = fs::read_to_string(&extension_toml_path)
        .unwrap()
        .parse()
        .unwrap();

    let version = value(env::var("CARGO_PKG_VERSION").unwrap());
    if version.to_string() != extension_toml["version"].to_string() {
        extension_toml["version"] = value(env::var("CARGO_PKG_VERSION").unwrap());
        fs::write(extension_toml_path, extension_toml.to_string()).unwrap();
    }
}
