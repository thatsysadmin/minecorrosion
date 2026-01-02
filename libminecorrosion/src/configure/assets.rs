use std::path::{Path, PathBuf};
use serde_json::Value;
use crate::breakpoint_trap_option;
use crate::configure::shared::extract_keys;
use crate::download_and_verify::Asset;

pub fn get_assets(container: Value) -> Option<Vec<Asset>> {
    let mut vector: Vec<Asset> = Vec::new();
    let objects = breakpoint_trap_option(container.get("objects"))?;
    for key in extract_keys(objects) {
        let x = breakpoint_trap_option(objects.get(&key))?;
        let path = PathBuf::from(Path::new(key.as_str()));
        let hash = breakpoint_trap_option(breakpoint_trap_option(x.get("hash"))?.as_str())?;
        let size = breakpoint_trap_option(breakpoint_trap_option(x.get("size"))?.as_u64())?;
        let y = Asset {
            filepath: path,
            hash: hash.to_string(),
            size,
        };
        vector.push(y);
    }
    Some(vector)
}

pub fn download_assets(client: &reqwest::blocking::Client, container: Vec<Asset>) {
    println!("Downloading assets.");

    for asset in container {
        println!()
    }
}