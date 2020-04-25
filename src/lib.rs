#![allow(unused)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod constants;
mod html_parser;
use chrono::prelude::*;

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
