use crate::{Verbs};

use anyhow::{Result, Context};
use reqwest::blocking::Response;
use reqwest::StatusCode;
use serde_json::{Value, Error, from_str};
use std::{
    env::var,
    io::{Write, Read},
    process::Command,
};
use tempfile::NamedTempFile;
use std::process::exit;

pub const VERSION: &str = "0.1";
pub const COMPATIBLE_DROGUE_VERSION: &str = "0.3.0";

pub fn print_result(res: Result<Response, reqwest::Error>, resource_name: String, op: Verbs) {
    match res {
        Ok(r) => {
            match op {
                Verbs::create => {
                    match r.status() {
                        StatusCode::CREATED => println!("{} created.", resource_name),
                        r => println!("Error : {}", r),
                    }
                }, Verbs::delete => {
                    match r.status() {
                        StatusCode::NO_CONTENT => println!("{} deleted.", resource_name),
                        r => println!("Error : {}", r),
                    }
                }, Verbs::get => {
                    match r.status() {
                        StatusCode::OK => println!("{}", r.text().expect("Empty response")),
                        r => println!("Error : {}", r),
                    }
                }, Verbs::edit => {
                    match r.status() {
                        StatusCode::NO_CONTENT => println!("{} edited.", resource_name),
                        r => println!("Error : {}", r),
                    }
                }
            }
        },
        Err(e) => println!("Error sending request : {}", e)
    }
}

pub fn json_parse(data: Option<&str>) -> Result<Value, Error> {
    from_str(data.unwrap_or("{}"))
}


pub fn editor(original: String) -> Result<Value> {

    //TODO : that would not work on windows !
    let editor = var("EDITOR").unwrap_or("vi".to_string());
    let file = NamedTempFile::new()?;
    //the handler needs to be kept to reopen the file later.
    let mut file2= file.reopen()?;

    // Write the original data to the file.
    file.as_file().write_all(original.as_bytes())?;

    Command::new(editor)
         .arg(file.path())
         .status()
         .expect("Could not open current data in editor.");

    // Read the data using the second handle.
    let mut buf = String::new();
    file2.read_to_string(&mut buf)?;

    from_str(buf.as_str()).with_context(|| "Invalid JSON data.")
}

pub fn print_version() {
   //todo add git hash and build date to version output ?

    println!("Client Version: {}", VERSION);
    println!("Compatible Server Version: {}", COMPATIBLE_DROGUE_VERSION);
    //todo connect to server and retrieve version.

    exit(0);
}