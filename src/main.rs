extern crate chrono;
extern crate term;
extern crate toml;

extern crate gumdrop;

use gumdrop::Options;

use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use chrono::Datelike;

#[macro_use]
extern crate serde;

const CARGO_FILE: &str = "Cargo.toml";

#[derive(Debug, Deserialize)]
struct ManifestInfo {
    package: PackageInfo,
}

#[derive(Debug, Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
    authors: Vec<String>,
    description: Option<String>,
    repository: Option<String>,
    keywords: Option<Vec<String>>,
}

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

impl PackageInfo {
    pub fn build_bibtex(&self) -> String {
        let mut buf = String::from("@misc{");
        buf = buf + self.name.as_str() + ",\n";
        buf = buf + "\ttitle={{" + self.name.as_str();
        buf = match &self.description {
            None => buf,
            Some(s) => buf + ": " + s.as_str(),
        } + "}},\n";
        buf = buf + "\tauthor={{" + self.authors.join(" and ").as_str() + "}},\n";
        let t = chrono::prelude::Local::now();
        let month = t.month();
        let year = t.year();

        let month_fmt = format!("\tmonth = {:0},\n", month);
        let year_fmt = format!("\tyear = {:0},\n", year);
        buf = buf + month_fmt.as_str() + year_fmt.as_str();
        buf = match &self.repository {
            None => buf,
            Some(url) => buf + "\turl = {{" + url.as_str() + "}}\n",
        };
        buf = match &self.keywords {
            None => buf,
            Some(keywords) => buf + "\tkeywords = {" + keywords.join(", ").as_str() + "}\n",
        };
        buf + "}\n"
    }
}

fn main() {
    let opt = CitationOption::parse_args_default_or_exit();
    let dir = env::current_dir().unwrap();
    let cargo_path = PathBuf::from(CARGO_FILE);
    let abs_path = dir.join(cargo_path);

    if !abs_path.exists() {
        println!("Current directory is not a Cargo project.");
        return;
    }
    let mut file = match fs::File::open(&abs_path) {
        Err(_) => panic!("Unable to read Cargo file from {:?}", &abs_path),
        Ok(file) => file,
    };
    let mut cargo_content = String::new();
    if let Err(e) = file.read_to_string(&mut cargo_content) {
        panic!(e);
    }
    let t: ManifestInfo = toml::from_str(cargo_content.as_str()).unwrap();
    println!("t: {:?}", t);
    let r = t.package.build_bibtex();
    println!("{}", r);
}
