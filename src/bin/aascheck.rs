// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
// SPDX-FileCopyrightText: 2025 Jan Hecht
//
// SPDX-License-Identifier: MIT

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::Parser;
use color_eyre::eyre::{Context, Result};
use colored::Colorize;
use jsonschema::Validator;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "Fraunhofer Institute for Experimental Software Engineering IESE"
)]
struct Opts {
    #[clap(parse(from_os_str))]
    input: PathBuf,
    #[clap(short, long)]
    mode: Mode,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
enum Mode {
    AAS,
    Submodel,
}

impl FromStr for Mode {
    type Err = AASCheckError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "aas" => Ok(Mode::AAS),
            "submodel" => Ok(Mode::Submodel),
            _ => Err(AASCheckError::InvalidMode(s.to_string())),
        }
    }
}

#[derive(Error, Debug)]
pub enum AASCheckError {
    #[error("{0}")]
    InvalidMode(String),
}

fn main() -> Result<()> {
    println!("Information:");
    println!("The original JSON schema uses regex, that contain unicode surrogates.");
    println!("The schema that is used for `aascheck` has all unicode surrogate-containing regex removed.");

    let opt = Opts::parse();
    let instance = read_json(&opt.input)?;
    let schema = static_json()?;
    let compiled = jsonschema::draft201909::new(&schema)?;
    check(&compiled, instance, opt.mode)?;
    output(&opt.input, opt.mode);
    Ok(())
}

fn check(schema: &Validator, instance: Value, mode: Mode) -> Result<()> {
    let instance = match mode {
        Mode::AAS => instance,
        Mode::Submodel => {
            json!({
                "assetAdministrationShells": [],
                "submodels": [
                    instance
                ],
                "conceptDescriptions": []
            })
        }
    };
    let result = match schema.validate(&instance) {
        Ok(_) => Ok(()),
        Err(validation_error) => {
            let text = validation_error.to_string();
            // .into_iter() !!the new validate function returns only the first error, so nothing to iterate
            // consider using iter_errors in the future
            // .map(|e| format!("{:#?}", e))
            // .collect::<Vec<String>>()
            // .join("\n");
            Err(color_eyre::eyre::anyhow!(text).wrap_err("Validation failed!"))
        }
    };
    result
}

fn read_json(path: &Path) -> Result<serde_json::Value> {
    let content =
        std::fs::read_to_string(path).wrap_err(format!("Opening file: {}", path.display()))?;
    Ok(serde_json::from_str(&content)?)
}

fn static_json() -> Result<serde_json::Value> {
    let content = include_str!("../../schema/aas.json");
    Ok(serde_json::from_str(content)?)
}

fn output(path: &Path, mode: Mode) {
    let t = match mode {
        Mode::AAS => "Asset Administration Shell",
        Mode::Submodel => "Submodel of an Asset Administration Shell",
    }
    .bold();
    let path = format!("{}", path.display()).bold();
    let text = format!("{} is a valid {}", path, t).green();
    println!("{}", text);
}
