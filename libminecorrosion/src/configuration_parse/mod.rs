use std::fs;
use std::path::PathBuf;
use serde_json::Value;

mod main;
mod download_executables;
mod download_assets;
mod runtime_variables;

pub fn parse_configuration(
    vanilla_configuration: &PathBuf,
    fabric_configuration: &Option<PathBuf>,
) {
    let vanilla_json = json_extractor(vanilla_configuration);
    
    
    
    match fabric_configuration {
        Some(fabric_configuration) => { // We need to write some overrides if the fabric configuration exist.
            let fabric_json = json_extractor(fabric_configuration);
            
            
        }
        None => {} // Potentially do nothing here.
    }

    println!("break");
}

fn json_extractor(path: &PathBuf) -> JsonExtractorOption {
    let reader = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return JsonExtractorOption::FSreaderIssue
    };
    let json: Value = match serde_json::from_str(&reader) {
        Ok(j) => j,
        Err(_) => return JsonExtractorOption::JsonParseIssue
    };
    JsonExtractorOption::OK(json)
}

enum JsonExtractorOption {
    OK(Value),
    FSreaderIssue,
    JsonParseIssue,
}

impl JsonExtractorOption {
    fn unwrap(self) -> Value {
        match self {
            JsonExtractorOption::OK(v) => v,
            JsonExtractorOption::FSreaderIssue => panic!("Having trouble reading JSON file."),
            JsonExtractorOption::JsonParseIssue => panic!("Having trouble parsing JSON file."),
        }
    }
}