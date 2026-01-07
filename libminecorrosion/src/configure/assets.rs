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
use crate::configure::shared::{extract_keys, verify_file, DownloadResult, DownloadResultReason};

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
) -> DownloadResult {
    println!("Downloading assets.");
    let Some(_) = breakpoint_trap_option(fs::create_dir_all(download_root).ok()) else { return DownloadResult::Failure };
    let path_hash_layout = download_root.join("hash");
    let Some(_) = breakpoint_trap_option(fs::create_dir_all(&path_hash_layout).ok()) else { return DownloadResult::Failure };
    let path_sane_layout = download_root.join("sane");
    let Some(_) = breakpoint_trap_option(fs::create_dir_all(&path_sane_layout).ok()) else { return DownloadResult::Failure };

    let mut artifact_status: Vec<(String, DownloadResultReason)> = Vec::new();
    let download_list_length = container.len();
    let mut successes: usize = 0;

    for asset in container {
        let download_sane_path = path_sane_layout.join(&asset.filepath);
        let hash_first_two = asset.hash.as_str().chars().take(2).collect::<String>();
        let download_hash_path = path_hash_layout.join(&hash_first_two).join(&asset.hash);
        let url = format!("{}/{}/{}", RESOURCES_ENDPOINT, hash_first_two, asset.hash);
        let path_str = match asset.filepath.to_str() {
            None => { return DownloadResult::Failure },
            Some(x) => { x }
        };

        let Some(_) = breakpoint_trap_option(fs::create_dir_all(
            match breakpoint_trap_option(download_sane_path.parent()) {
                None => { return DownloadResult::Failure }
                Some(x) => { x }
            }
        ).ok()) else { return DownloadResult::Failure };
        let Some(_) = breakpoint_trap_option(fs::create_dir_all(
            match breakpoint_trap_option(download_hash_path.parent()) {
                None => { return DownloadResult::Failure }
                Some(x) => { x }
            }
        ).ok()) else { return DownloadResult::Failure };

        // Check for file before downloading it again
        print!("Getting {} - ", url);
        let download_file;
        if download_sane_path.exists() {
            download_file = match verify_file(&download_sane_path, &asset.hash) {
                None => { return DownloadResult::Failure }
                Some(x) => { x }
            };
        }
        else {
            download_file = true;
        }

        if download_file {
            let reqwest = client.get(url.as_str());
            let result = match reqwest.send() {
                Ok(x) => { x }
                Err(_) => { return DownloadResult::Failure }
            };
            let result_code = result.status();
            if result_code == reqwest::StatusCode::OK {
                let body = match result.bytes() {
                    Ok(x) => { x }
                    Err(_) => { return DownloadResult::Failure }
                };

                let mut hasher = Sha1::new();
                hasher.update(&body);
                let hasher_result = hasher.finalize();
                let hasher_ascii = format!("{:x}", hasher_result);
                if hasher_ascii == asset.hash { // TODO: This might be a security risk, need to look into this?
                    artifact_status.push((path_str.to_string(), DownloadResultReason::Success));
                    println!("{}", "Success, passed SHA1 hash check.".bright_green());
                    successes += 1;
                }
                else {
                    artifact_status.push((path_str.to_string(), DownloadResultReason::FailedChecksum));
                    println!("{}", "Success, failed SHA1 hash check.".bright_yellow());
                    panic!()
                }
                let mut file_handler = match breakpoint_trap_option(File::create(&download_sane_path).ok()) {
                    None => { return DownloadResult::Failure }
                    Some(x) => { x }
                };
                let Some(_) = breakpoint_trap_option(file_handler.write_all(&body).ok()) else { return DownloadResult::Failure };

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
                        let Ok(_) = os::unix::fs::symlink(download_sane_path_relative, download_hash_path) else { return DownloadResult::Failure };
                    }
                }
                #[cfg(target_family = "windows")]
                {
                    panic!("FIX THIS."); // TODO: Fix this later.
                    let download_sane_path_relative = Path::new("..\\..").join(&asset.filepath);
                    let Ok(_) = os::windows::fs::symlink_file(download_sane_path_relative, download_hash_path) else { return DownloadResult::Failure };
                }
            }
            else if result_code == reqwest::StatusCode::NOT_FOUND {
                artifact_status.push((path_str.to_string(), DownloadResultReason::FailedDownload(StatusCode::NOT_FOUND)));
                println!("{}", "Not found.".bright_red());
            }
            else {
                artifact_status.push((path_str.to_string(), DownloadResultReason::FailedDownload(result_code)));
                println!("{} {}, panicking.", "Got other status code: ".bright_red(), result_code);
            }
        }
    }
    if download_list_length == successes {
        DownloadResult::Success
    }
    else {
        DownloadResult::SuccessWithIssues(artifact_status)
    }
}