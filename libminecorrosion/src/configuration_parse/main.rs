use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;
use crate::configuration_parse::parse_configuration;

fn main() {
    println!("configuration_parse");

    let command_arguments = env::args().collect::<Vec<String>>();
    let configuration_filepaths: Vec<PathBuf> = Vec::new();
    for argument in command_arguments {
        if argument == "-h" || argument == "--help" {
            println!("minecorrosion: configuration_parse");
            println!("Add files as arguments. Dependancies matter.");
            println!("Ex: A fabric configuration needs to be before a vanilla profile.");
            exit(0);
        }
        let vanilla_config_path = PathBuf::from("/Volumes/tmpfs/minecorrosion/example_json/1.21.10_pretty.json");
        let fabric_configuration = PathBuf::from("/Volumes/tmpfs/minecorrosion/example_json/fabric-0.17.3-1.21.10.json");
        
        parse_configuration(&vanilla_config_path, &Some(fabric_configuration));
    }
}