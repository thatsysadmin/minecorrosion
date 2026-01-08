use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use serde_json::Value;
use sha1::{Digest, Sha1};
use crate::breakpoint_trap_option;

const OPENING_DELINEATOR: &str = "${";
const CLOSING_DELINEATOR: &str = "}";

pub fn lookup_substitution<'a>(argument: &'a str, environment_variable: &std::collections::HashMap<&str, &'a str>) -> Option<String> {
    if argument.contains(OPENING_DELINEATOR) && argument.contains(CLOSING_DELINEATOR) {
        let regex = Regex::new(r"\$\{(.*?)}").unwrap();
        let query = regex.find(argument).unwrap().as_str();
        let stripped = query // TODO: we're gonna need to replace this at some point
            .replace("$", "")
            .replace("{", "")
            .replace("}", "");
        let value = match environment_variable.get(stripped.as_str()) {
            Some(x) => x,
            None => {
                return None;
            }
        };
        Some(regex.replace(argument, *value).to_string())
    }
    else {
        Some(argument.to_string())
    }
}

// We need to extract the keys from features.
// I guess it made sense from their perspective somehow,
// but I'm not too sure how they're doing it.
// There's only one rule in client versions, but if
// Mojang adds some more stuff then this will probably break.
// Please excuse this possibly horrible mess.
pub fn extract_keys(x: &serde_json::value::Value) -> Vec<String> {
    // Anything below is pretty questionable IMO. Prepare to be bewildered.
    let mut keys: Vec<String> = Vec::new();
    let object = x.as_object().unwrap().keys();
    for i in object {
        let key: String = i.clone().parse().unwrap();
        keys.push(key);
    }
    // if keys.len() != 1 {
    //     panic!()
    // }
    keys
}

pub fn process_rule(rule: &serde_json::value::Value, rules: &HashMap<&str, bool>) -> Option<bool> {
    let action = match rule.get("action") {
        None => {
            panic!()
        }
        Some(x) => { x }
    };

    if action == "allow" {
        let mut os = std::env::consts::OS;
        if os == "macos" {
            os = "osx"; // mojang still uses the term "osx" so we need to account for that.
        }
        let arch = std::env::consts::ARCH;

        let features = match rule.get("features") {
            None => {
                // Test to see if there's an OS rule instead.
                let mut rtn = false;

                let os_rule = breakpoint_trap_option(rule.get("os"))?;

                // seems to be either name or arch.
                let keys = extract_keys(os_rule);
                if keys.len() != 1 {
                    return None
                }
                let option = &keys[0];
                if option == "name" { // actually the platform for some reason
                    let value = os_rule.get(option).unwrap();
                    rtn = value == os;
                }
                else if option == "arch" {
                    let value = os_rule.get(option).unwrap();
                    rtn = value == arch;
                }
                else {
                    return None
                }

                rtn
            }
            Some(x) => {
                let keys = extract_keys(x);
                if keys.len() != 1 {
                    return None
                }
                match rules.get(keys[0].as_str()) {
                    None => {
                        false
                    }
                    Some(z) => { *z }
                }
            }
        };
        Some(features)
    }
    else if action == "disallow" {
        Some(false)
    }
    else {
        // panic!()
        None
    }
}

pub fn verify_file(path: &Path, provided_hash: &str) -> Option<bool> {
    let file_contents = fs::read(&path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&file_contents);
    let hasher_result = &hasher.finalize();
    let hasher_ascii = format!("{:x}", hasher_result);

    if hasher_ascii == provided_hash {
        println!("File already downloaded, SHA1 checksum verified.");
        Some(false)
    }
    else {
        println!("File failed verification, redownloading.");
        Some(true)
    }
}

pub enum DownloadResult {
    Success,
    SuccessWithIssues(Vec<(String, DownloadResultReason)>),
    Failure,
    FailedPrerequisiteOperation()
}

pub enum DownloadResultReason {
    Success,
    FailedChecksum,
    FailedDownload(reqwest::StatusCode),
}