use std::collections::HashMap;
use std::error::Error;
use serde_json::Value;
use crate::breakpoint_trap_option;
use crate::configure::shared::{lookup_substitution, extract_keys, process_rule};

pub fn parse_arguments_game_plus_jvm(
    element_container_x: &serde_json::value::Value,
    environment_variable: &HashMap<&str, &str>,
    rules: &HashMap<&str, bool>
) -> Option<Vec<String>> {
    // OS variables plus hacks
    let mut os = std::env::consts::OS;
    if os == "macos" {
        os = "osx"; // mojang still uses the term "osx" so we need to account for that.
    }
    let arch = std::env::consts::ARCH;

    let mut arguments: Vec<String> = Vec::new();

    let element_container = breakpoint_trap_option(element_container_x.as_array())?;
    for element in element_container {
        if element.is_string() {
            let argument = breakpoint_trap_option(element.as_str())?;
            let ls = lookup_substitution(argument, environment_variable);
            arguments.push(ls);
        }
        else if element.is_object() { // "rule" element
            let rule_container = match element.get("rules") {
                None => {
                    panic!()
                }
                Some(x) => {
                    match x.as_array() {
                        None => {
                            panic!()
                        }
                        Some(y) => { y }
                    }
                }
            };

            for rule in rule_container {
                if process_rule(rule, rules) { // True
                    let arguments_value = match element.get("value") {
                        None => {
                            panic!()
                        }
                        Some(x) => { x }
                    };
                    match arguments_value {
                        Value::String(x) => {
                            let y = lookup_substitution(x.as_str(), &environment_variable);
                            arguments.push(y);
                        }
                        Value::Array(x) => {
                            for y in x {
                                let z = match y.as_str() {
                                    None => {
                                        panic!()
                                    }
                                    Some(b) => { b }
                                };
                                let a = lookup_substitution(z, &environment_variable);
                                arguments.push(a);
                            }
                        }
                        _ => {
                            panic!()
                        }
                    }
                }
                // else -> ignore
            }
        }
        else {
            panic!()
        }
    } // for element in element_container
    arguments
} // root function