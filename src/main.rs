#![deny(missing_docs)]
//! Launch and display a single json objects file analysis
//! This tool needs an input file containing a json object on each line, including a field `type`.
//! An analysis result will be displayed as an aggregation of some infos for each type.

mod parsing_result;

use crate::parsing_result::{Analysis, TypeInfo};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::ffi::OsStr;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde_json;
extern crate term_table;


/// Check that json_object is of the correct format to be interpreted and parse it
/// # Errors
/// If there is a format error, simply log it
fn interpret_single_object(file_analysis: &mut Analysis, json_object: &serde_json::Map<String, serde_json::Value>, num: usize, byte_size: usize) -> () {
    // check that json object contains type field
    // could further verify with pattern matching that field is string if this constraint is needed
    if let Some(json_type) = json_object.get("type") {
        // waiting for https://github.com/rust-lang/rust/issues/53667 to chain if let
        if let Ok(json_type_str) = serde_json::to_string(json_type) {
            let type_info = file_analysis.get_data().entry(json_type_str).or_insert(TypeInfo::new());
            type_info.add_object(byte_size);
        } else {
            error!("invalid type info for json entry on line {}", num)
        }
    } else {
        error!("missing type info for json entry on line {}", num)
    }
}

/// Construct an [Analysis](parsing_result::Analysis) by parsing each line of the input file
/// # Errors
/// Returns an [std::io::Error] on file manipulation error (open, read)
/// else simply log the error if it comes from the json interpretation of a single line
fn get_analysis(input_filename: &OsStr) -> Result<Analysis, std::io::Error> {
    let file = File::open(input_filename)?;
    let file = BufReader::new(file);
    let mut file_analysis = Analysis::new();

    for (num, line_res) in file.lines().enumerate() {
        let line = line_res?;
        if line.is_empty() {
            warn!("empty entry on line {}", num+1);
            continue;
        }
        // parse json and tell it to expect an object (serde_json::Map)
        // an error will appears if json on that line is bad or is anything else than an object
        let parsing_result: serde_json::Result<serde_json::Map<String, serde_json::Value>> = serde_json::from_str(&line);
        match parsing_result {
            Ok(json_object) => {
                interpret_single_object(&mut file_analysis, &json_object, num+1, line.len());
            }
            Err(err) => {
                match err.classify() {
                    serde_json::error::Category::Syntax |
                    serde_json::error::Category::Data |
                    serde_json::error::Category::Eof => {
                        error!("unexpected json entry format on line {}", num+1)
                    }
                    _ => return Err(err.into()),
                }
            }
        }
    }

    Ok(file_analysis)
}

/// Parse an input file and display its analysis
/// or log on error
fn analyse_file(input_filename: &OsStr) -> () {
    match get_analysis(input_filename) {
        Ok(analysis) => {
            print!("{:#?}", analysis); // pretty print
        }
        Err(err) => error!("{}", err)
    }
}

fn main() -> () {
    env_logger::init();

    // Command line arguments parsing
    let matches = clap_app!(mini_rust_parser =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: "Parse a file and display an analysis of collected data")
        (@arg INPUT_FILE: +required "Path to the file to parse")
    ).get_matches();
    // Calling .unwrap() is safe here because "INPUT_FILE" is required
    let input_filename = matches.value_of_os("INPUT_FILE").unwrap();

    analyse_file(input_filename);
}