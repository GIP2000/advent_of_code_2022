use anyhow::{Context, Result};
use std::{fs::read_to_string, str::FromStr};
pub fn parse_file<T: FromStr>(file_path: &str) -> Result<Vec<T>> {
    let file_content = read_to_string(file_path)?;
    parse_string(&file_content)
}

pub fn parse_string<T: FromStr>(content: &str) -> Result<Vec<T>> {
    content
        .split("\n")
        .map(|line| {
            line.trim()
                .parse::<T>()
                .ok()
                .context("Error Parsing string")
        })
        .collect::<Result<Vec<_>>>()
}
