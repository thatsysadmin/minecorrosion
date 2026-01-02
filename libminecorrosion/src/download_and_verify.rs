use std::path::{Path, PathBuf};

pub fn download_and_verify(client: &reqwest::blocking::Client, asset: Asset, download_path: PathBuf) -> bool {
    
    
    panic!()
}


pub struct Asset {
    pub filepath: PathBuf,
    pub hash: String,
    pub size: u64,
}