#[macro_use]
extern crate pest_derive;
extern crate pest;

mod oserror;
mod unveil;

use std::env;
use std::fs;
use std::str;
use pest::Parser;
use unveil::unveil;

const VEIL_RULES_PATH: &str = "VEIL_RULES_PATH";

#[derive(Parser)]
#[grammar = "rules.pest"]
pub struct RulesParser;

#[no_mangle]
#[link_section = ".init_array"]
static veil_init: fn() = veil;

fn veil() {
    let rules_path = match env::var(VEIL_RULES_PATH) {
        Ok(v) => v,
        Err(_e) => {
            eprintln!("VEIL_RULES_PATH not set");
            return;
        }
    };
    let rules_string = match fs::read_to_string(rules_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Unable to open rules file: {}", e);
            return;
        }
    };
    let file = match RulesParser::parse(Rule::file, &rules_string) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to parse rules: {}", e);
            return;
        }
    }
    .next()
    .unwrap();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::line => {
                let mut pairs = line.into_inner();
                let path = pairs.next().unwrap().as_str();
                let permissions = pairs.next().unwrap().as_str();

                unveil(Some(path), Some(permissions)).unwrap_or_else(|e| {
                    eprintln!("Failed to unveil {}: {}", path, e);
                });
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    unveil(None, None).unwrap_or_else(|e| {
        eprintln!("Failed to disable future calls to unveil: {}", e);
    });
}
