use std::{fs, os};
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::path::{Path, PathBuf};
use colored::Colorize;
use reqwest::StatusCode;
use serde_json::Value;
use sha1::{Digest, Sha1};
use crate::{breakpoint_trap_option, breakpoint_trap_result};
use crate::configure::libraries::DownloadLibrariesResultReason;
use crate::configure::shared::{extract_keys, verify_file};

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

pub struct Asset {
    pub filepath: PathBuf,
    pub hash: String,
    pub size: u64,
}

const RESOURCES_ENDPOINT: &str = "https://resources.download.minecraft.net";
pub fn download_assets(
    client: &reqwest::blocking::Client,
    container: Vec<Asset>,
    download_root: &Path
) -> Option<()> {
    println!("Downloading assets.");
    breakpoint_trap_option(fs::create_dir_all(download_root).ok())?;
    let path_hash_layout = download_root.join("hash");
    breakpoint_trap_option(fs::create_dir_all(&path_hash_layout).ok())?;
    let path_sane_layout = download_root.join("sane");
    breakpoint_trap_option(fs::create_dir_all(&path_sane_layout).ok())?;

    let mut download_results: Vec<(String, DownloadLibrariesResultReason)> = Vec::new();

    for asset in container {
        let download_sane_path = path_sane_layout.join(&asset.filepath);
        let hash_first_two = asset.hash.as_str().chars().take(2).collect::<String>();
        let download_hash_path = path_hash_layout.join(&hash_first_two).join(&asset.hash);
        let url = format!("{}/{}/{}", RESOURCES_ENDPOINT, hash_first_two, asset.hash);
        let path_str = asset.filepath.to_str().unwrap();

        breakpoint_trap_option(fs::create_dir_all(breakpoint_trap_option(download_sane_path.parent())?).ok())?;
        breakpoint_trap_option(fs::create_dir_all(breakpoint_trap_option(download_hash_path.parent())?).ok())?;

        // Check for file before downloading it again
        print!("Getting {} - ", path_str);
        let download_file;
        if download_sane_path.exists() {
            download_file = verify_file(&download_sane_path, &asset.hash)?;
        }
        else {
            download_file = true;
        }

        if download_file {
            let reqwest = client.get(url.as_str());
            let result = reqwest.send().unwrap();
            let result_code = result.status();
            if result_code == reqwest::StatusCode::OK {
                let body = result.bytes().unwrap();

                let mut hasher = Sha1::new();
                hasher.update(&body);
                let hasher_result = hasher.finalize();
                let hasher_ascii = format!("{:x}", hasher_result);
                if hasher_ascii == asset.hash { // TODO: This might be a security risk, need to look into this?
                    download_results.push((path_str.to_string(), DownloadLibrariesResultReason::Success));
                    println!("{}", "Success, passed SHA1 hash check.".bright_green());
                }
                else {
                    download_results.push((path_str.to_string(), DownloadLibrariesResultReason::FailedChecksum));
                    println!("{}", "Success, failed SHA1 hash check.".bright_yellow());
                    panic!()
                }
                let mut file_handler = breakpoint_trap_option(File::create(&download_sane_path).ok())?;
                breakpoint_trap_option(file_handler.write_all(&body).ok())?;

                // symlink off the sane folder to the hash file structure
                #[cfg(target_family = "unix")]
                {
                    let download_sane_path_relative = Path::new("../..").join(&asset.filepath);
                    let create_symlink: bool;
                    if !download_hash_path.exists() {
                        create_symlink = false;
                    }
                    else if !download_hash_path.is_symlink() {
                        let symlink_hash_path = Path::new(&download_hash_path);
                        create_symlink = symlink_hash_path != download_sane_path_relative;
                    }
                    else {
                        create_symlink = true;
                    }

                    if create_symlink {
                        os::unix::fs::symlink(download_sane_path_relative, download_hash_path).unwrap();
                    }
                }
                #[cfg(target_family = "windows")]
                {
                    panic!("FIX THIS."); // TODO: Fix this later.
                    let download_sane_path_relative = Path::new("..\\..").join(&asset.filepath);
                    os::windows::fs::symlink_file(download_sane_path_relative, download_hash_path).unwrap();
                }
            }
            else if result_code == reqwest::StatusCode::NOT_FOUND {
                download_results.push((path_str.to_string(), DownloadLibrariesResultReason::FailedDownload(StatusCode::NOT_FOUND)));
                panic!()
            }
            else {
                download_results.push((path_str.to_string(), DownloadLibrariesResultReason::FailedDownload(result_code)));
                panic!()
            }
        }

        // println!()
    }
    panic!()
}