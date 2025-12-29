use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use reqwest::StatusCode;
use sha1::{Digest, Sha1};
use colored::Colorize;
use crate::{breakpoint_trap_option, breakpoint_trap_result};
use crate::configure::shared::{extract_keys, process_rule};

pub fn get_libraries(configuration: &serde_json::value::Value, os: &str, rules: HashMap<&str, bool>) -> Option<Vec<ArtifactStructure>> {
    let configuration_vector = breakpoint_trap_option(configuration.as_array())?;

    let mut artifacts: Vec<ArtifactStructure> = Vec::new();
    for library_entry in configuration_vector {
        let mut rule = true;
        match library_entry.get("rules") {
            None => {
                // Do nothing - fail true.
            }
            Some(element) => {
                // unwrap
                let internal = breakpoint_trap_option(element.as_array())?;
                rule = breakpoint_trap_option(process_rule(&internal[0], &rules))?;
            }
        }

        if rule {
            let x = breakpoint_trap_option(parse_artifact(library_entry))?;
            artifacts.push(x);
        }
    }

    Some(artifacts)
}

fn parse_artifact(value: &serde_json::value::Value) -> Option<ArtifactStructure> {
    let container_x = breakpoint_trap_option(value.get("downloads"))?;
    let container_y = breakpoint_trap_option(container_x.get("artifact"))?;
    Some(ArtifactStructure {
        path: breakpoint_trap_option(container_y.get("path"))?.to_string().replace("\"", ""),
        sha1: breakpoint_trap_option(container_y.get("sha1"))?.to_string().replace("\"", ""),
        size: breakpoint_trap_option(breakpoint_trap_option(container_y.get("size"))?.as_u64())?,
        url: breakpoint_trap_option(container_y.get("url"))?.to_string().replace("\"", "")
    })
}

pub struct ArtifactStructure {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

pub async fn download_libraries(client: reqwest::Client, download_list: Vec<ArtifactStructure>, downloadroot_str: &str) -> Option<Vec<(String, bool)>> {
    let mut downloadroot = PathBuf::new();
    downloadroot.push(Path::new(downloadroot_str));

    let mut artifact_status: Vec<(String, bool)> = Vec::new();
    let mut successes: usize = 0;
    let download_list_length = download_list.len();

    for artifact in download_list {
        let relative_filepath = Path::new(artifact.path.as_str());
        let mut absolute_path = PathBuf::new();
        absolute_path.push(&downloadroot);
        absolute_path.push(relative_filepath);
        let mut folder_path = absolute_path.clone();
        folder_path.pop();

        print!("Getting {} - ", artifact.url);
        let download_file;

        // Check to see if we have the file, and if we do, verify it.
        let file_exists = absolute_path.exists();
        if file_exists {
            let file_contents = fs::read(&absolute_path).unwrap();
            let mut hasher = Sha1::new();
            hasher.update(&file_contents);
            let hasher_result = &hasher.finalize();
            let hasher_ascii = format!("{:x}", hasher_result);

            if hasher_ascii != artifact.sha1 {
                println!("Fine failed verification, redownloading.");
                download_file = true;
            }
            else {
                println!("File already downloaded, SHA1 checksum verified.");
                download_file = false;
            }
        }
        else {
            download_file = true;
        }

        if download_file { // Download the file.
            fs::create_dir_all(folder_path).unwrap();
            let reqwest = client.get(artifact.url.as_str());
            let result = reqwest.send().await.unwrap();
            let reqwest_code = result.status();
            if reqwest_code == StatusCode::OK {
                artifact_status.push((artifact.path, true));
                successes += 1;
                let body = result.bytes().await.unwrap();

                // Verify the sha1 hash
                let mut hasher = Sha1::new();
                hasher.update(&body);
                let hasher_result = &hasher.finalize();
                let hasher_ascii = format!("{:x}", hasher_result);
                if hasher_ascii == artifact.sha1 {
                    println!("{}", "Success, passed SHA1 hash check.".bright_green());
                } else {
                    println!("{}", "Success, failed SHA1 hash check.".bright_yellow());
                }

                let mut file_handler = File::create(&absolute_path).unwrap();
                file_handler.write_all(&body).unwrap();
            } else if reqwest_code == StatusCode::NOT_FOUND {
                artifact_status.push((artifact.path, false));
                println!("{}", "Not found.".bright_red());
            } else {
                println!("{} {}, panicking.", "Got other status code: ".bright_red(), reqwest_code);
                panic!()
            }
        }
    }

    println!("Done downloading libraries.");
    if successes == download_list_length {
        None
    }
    else {
        Some(artifact_status)
    }
}