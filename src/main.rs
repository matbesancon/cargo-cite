extern crate term;
extern crate toml;
extern crate chrono;

#[macro_use]
extern crate gumdrop;

use gumdrop::Options;

use std::env;
use std::path::{Path, PathBuf};
use std::path;
use std::fs;
use std::io::Read;

use core::fmt::Debug;
use chrono::Datelike;
use std::ops::Add;

const CARGO_FILE: &str = "Cargo.toml";

#[derive(Debug, Options)]
struct CitationOption {
    // Boolean options are treated as flags, taking no additional values.
    // The optional `help` attribute is displayed in `usage` text.
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "Generate CITATION.bib file")]
    generate: bool,

    #[options(help = "Over-write existing CITATION.bib file")]
    overwrite: bool,
}

fn main() {
    let opt = CitationOption::parse_args_default_or_exit();
    let dir = env::current_dir().unwrap();
    let cargo_path = PathBuf::from(CARGO_FILE);
    let abs_path = dir.join(cargo_path);

    println!("Hello, world!\n{:?}", opt);
    println!("Hello, world!\n{:?}", abs_path);
    if !abs_path.exists() {
        println!("Current directory is not a Cargo project.");
        return;
    }
    let mut file = match fs::File::open(&abs_path) {
        Err(_) => panic!("Unable to read Cargo file from {:?}", &abs_path),
        Ok(file) => file
    };
    let mut cargo_content = String::new();
    file.read_to_string(&mut cargo_content);
    println!("cargo content: {:}", cargo_content);
    let r = build_bibtex("cargocite", vec![String::from("Mathieu Besançon")], "v0.1.0", "https", "description");
    println!("{}", r);
}

fn build_bibtex(name: &str, authors: Vec<String>, version: &str, url: &str, description: &str) -> String {
    let mut buf = String::from("@misc{");
    buf = buf
        .add(name)
        .add(",\n");
    buf = buf
        .add("\ttitle={{")
        .add(name)
        .add(": ")
        .add(description)
        .add("}},\n");
    buf = buf
        .add("\tauthor={{")
        .add(authors.join(" and ").as_str())
        .add("}},\n");
    let t = chrono::prelude::Local::now();
    let month = t.month();
    let year  = t.year();

    let month_fmt = format!("\tmonth = {:0},\n", month);
    let year_fmt  = format!("\tyear = {:0},\n", year);
    buf = buf
        .add(month_fmt.as_str())
        .add(year_fmt.as_str());

    buf = buf
        .add("\turl = {{")
        .add(url)
        .add("}}\n");

    buf.add("}\n")
}