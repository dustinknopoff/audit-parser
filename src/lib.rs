#![allow(unused)]
//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use pest;
#[macro_use]
extern crate pest_derive;

mod constants;
mod html_parser;
use chrono::prelude::*;

/// Module for FFI Receiving/Freeing
pub mod ffi {
    use crate::html_parser::AuditParser;
    use std::{
        ffi::{CStr, CString},
        os::raw::c_char,
    };

    /// Given a pointer to a C-String, parse a NEU Web Audit
    #[no_mangle]
    pub extern "C" fn parse_web_audit_ffi(src: *const c_char) -> *mut c_char {
        let c_str = unsafe { CStr::from_ptr(src) };
        let recipient = match c_str.to_str() {
            Err(_) => "failed to convert from c string to rust string",
            Ok(string) => string,
        };
        let contents = AuditParser::parse_audit(recipient);
        let as_json = match serde_json::to_string_pretty(&contents.unwrap()) {
            Err(_) => "failed to convert to json".into(),
            Ok(val) => val,
        };
        CString::new(as_json)
            .expect("Could not convert in to cstring.")
            .into_raw()
    }

    #[no_mangle]
    /// Free a C-String
    pub extern "C" fn free_as_json(s: *mut c_char) {
        unsafe {
            if s.is_null() {
                return;
            }
            CString::from_raw(s)
        };
    }
}

pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html_parser::AuditParser;
    use fs::File;
    use pest::Parser;
    use serde_json::to_writer_pretty;
    use std::fs;

    // #[test]
    // fn nom_it_works() {
    //     use crate::html_parser::nom_parse::AuditToJson;
    //     use std::fs::File;
    //     use std::io::Read;
    //     let mut file = File::open("/Users/dustinknopoff/Downloads/Web Audit.txt").unwrap();
    //     let mut contents: String = String::new();
    //     file.read_to_string(&mut contents);
    //     let mut auditer = AuditToJson::new();
    //     let (input, skipped) = auditer.parse(&contents).unwrap();
    //     // dbg!(auditer);
    // }

    #[test]
    fn pest_it_works() {
        let unparsed_file = fs::read_to_string("/Users/dustinknopoff/Downloads/Web Audit.txt")
            .expect("cannot read file");

        let audit = AuditParser::parse_audit(&unparsed_file).unwrap();
        let mut output = File::create("/Users/dustinknopoff/Downloads/Web Audit.json").unwrap();
        to_writer_pretty(output, &audit).unwrap();
    }
}
