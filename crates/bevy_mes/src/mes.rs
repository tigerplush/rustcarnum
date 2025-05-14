use std::num::ParseIntError;

use bevy::{platform::collections::HashMap, prelude::*};
use regex::Regex;
use thiserror::Error;

#[derive(Asset, TypePath)]
pub struct Mes {
    pub contents: HashMap<u32, (Option<String>, String)>,
}

#[derive(Debug, Error)]
pub enum MesError {
    #[error("Regex pattern is erroneous")]
    RegexError(#[from] regex::Error),
    #[error("could not parse index")]
    ParseError(#[from] ParseIntError),
}

impl Mes {
    pub fn from_contents(raw_content: &str) -> Result<Mes, MesError> {
        let mut contents = HashMap::new();
        for line in raw_content.lines() {
            let pattern = r#"^\{(\d+)\}(.*?)\{(.*?)\}(.*?)$"#;
            let regex = Regex::new(pattern)?;
            if let Some(caps) = regex.captures(line) {
                let index: u32 = caps[1].parse()?;
                let optional = if let Some(m) = caps.get(2) {
                    if m.is_empty() {
                        None
                    } else {
                        Some(m.as_str().to_string())
                    }
                } else {
                    None
                };
                let content = caps
                    .get(3)
                    .map_or(String::new(), |m| m.as_str().to_string());
                contents.insert(index, (optional, content));
            }
        }
        Ok(Mes::new(contents))
    }

    fn new(contents: HashMap<u32, (Option<String>, String)>) -> Mes {
        Mes { contents }
    }
}
