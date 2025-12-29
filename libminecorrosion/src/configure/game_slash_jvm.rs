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
            let ls = breakpoint_trap_option(lookup_substitution(argument, environment_variable))?;
            arguments.push(ls);
        }
        else if element.is_object() { // "rule" element
            let rule_container_i = breakpoint_trap_option(element.get("rules"))?;
            let rule_container = breakpoint_trap_option(rule_container_i.as_array())?;

            for rule in rule_container {
                if breakpoint_trap_option(process_rule(rule, rules))? { // True
                    match breakpoint_trap_option(element.get("value"))? {
                        Value::String(x) => {
                            let y = breakpoint_trap_option(lookup_substitution(x.as_str(), environment_variable))?;
                            arguments.push(y);
                        }
                        Value::Array(x) => {
                            for y in x {
                                let z = breakpoint_trap_option(y.as_str())?;
                                let a = breakpoint_trap_option(lookup_substitution(z, environment_variable))?;
                                arguments.push(a);
                            }
                        }
                        _ => {
                            return None
                        }
                    }
                }
                // else -> ignore
            }
        }
        else {
            return None
        }
    } // for element in element_container
    Some(arguments)
} // root function