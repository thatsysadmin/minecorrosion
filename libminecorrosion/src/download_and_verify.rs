use std::path::{Path, PathBuf};
use colored::Colorize;
use reqwest::StatusCode;
use sha1::{Digest, Sha1};

pub fn download_and_verify(
    client: &reqwest::blocking::Client,
    hash: &String,
    size: &u64,
    filepath: &Path,
    download_url: &str
) -> Option<()> {
    let reqwest = client.get(download_url);
    let result = reqwest.send().unwrap();
    let reqwest_code = result.status();
    if reqwest_code == StatusCode::OK {
        let body = result.bytes().unwrap();
        
        // Verify the sha1 hash
        // let mut hasher = Sha1::new();
        // hasher.update(&body);
        // let hasher_result = &hasher.finalize();
        // let hasher_ascii = format!("{:x}", hasher_result);
        // if hasher_ascii == artifact.sha1 {
        //     successes += 1;
        //     artifact_status.push((artifact.path, crate::configure::libraries::DownloadLibrariesResultReason::Success));
        //     println!("{}", "Success, passed SHA1 hash check.".bright_green());
        // } else {
        //     artifact_status.push((artifact.path, crate::configure::libraries::DownloadLibrariesResultReason::FailedChecksum));
        //     println!("{}", "Success, failed SHA1 hash check.".bright_yellow());
        // }
    }
    else if reqwest_code == StatusCode::NOT_FOUND {

    }
    else {

    }

    panic!()
}