extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use rustc_serialize::base64::{FromBase64, ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::Read;
use std::string::String;

pub fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) {
        return "jpeg";
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {
        return "png";
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) {
        return "gif";
    }
    panic!("invalid file type")
}

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();
    format!(
        "data:image/{};base64,{}",
        get_file_type(&hex),
        base64.replace("\r\n", "")
    )
}

pub fn to_base64_vec(vec: Vec<u8>) -> String {
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();
    format!(
        "data:image/{};base64,{}",
        get_file_type(&hex),
        base64.replace("\r\n", "")
    )
}

pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    value.from_base64().unwrap()
}

