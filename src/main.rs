extern crate chrono;
extern crate term;
extern crate toml;

extern crate gumdrop;

use gumdrop::Options;

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

use chrono::Datelike;

#[macro_use]
extern crate serde;

const CARGO_FILE: &str = "Cargo.toml";
const CITATION_FILE: &str = "CITATION.bib";

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
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "Generate CITATION.bib file")]
    generate: bool,

    #[options(help = "Over-write existing CITATION.bib file")]
    overwrite: bool,

    #[options(
        help = "Append a \"Citing\" section to the README. Will create the file if not present."
    )]
    readme_append: bool,

    #[options(help = "Path to the crate, default to current directory")]
    path: Option<String>,

    #[options(help = "Citation file to add, default to CITATION.bib (recommended). \"STDOUT\" for outputing to standard output.")]
    filename: Option<String>,
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

    fn readme_section(&self) -> String {
        return String::from(
"
## Citing

If you found this software useful consider citing it. See CITATION.bib for the recommended BibTeX entry.
"
        );
    }
}

fn main() {
    let opt = CitationOption::parse_args_default_or_exit();

    let dir = match opt.path{
        Some(s) => PathBuf::from(s),
        None => env::current_dir().unwrap(),
    };

    let cargo_path = dir.join(PathBuf::from(CARGO_FILE));

    if !cargo_path.exists() {
        println!("Current directory is not a Cargo project.");
        return;
    }
    let mut cargo_file = match fs::File::open(&cargo_path) {
        Err(_) => panic!("Unable to read Cargo file from {:?}", &cargo_path),
        Ok(file) => file,
    };
    let mut cargo_content = String::new();
    if let Err(e) = cargo_file.read_to_string(&mut cargo_content) {
        panic!(e);
    }

    let t: ManifestInfo = toml::from_str(cargo_content.as_str()).unwrap();

    if opt.readme_append {
        for f in fs::read_dir(&dir).unwrap() {
            if let Ok(dir_entry) = f {
                let p = dir_entry.path();
                if String::from(p.to_str().unwrap()).contains("README") {
                    println!("Appending to readme file: {:?}", p);
                    match fs::OpenOptions::new().append(true).open(p) {
                        Ok(mut readme_file) => {
                            let readme_section = t.package.readme_section();
                            readme_file.write(readme_section.as_bytes());
                        }
                        Err(e) => println!("Error appending to readme file: {:?}", e),
                    }
                }
            }
        }
    }

    let t: ManifestInfo = toml::from_str(cargo_content.as_str()).unwrap();
    let r = t.package.build_bibtex();

    let output_file = if let Some(o) = opt.filename {
        o
    } else {
        String::from(CITATION_FILE)
    };
    let file_path = dir.join(PathBuf::from(output_file.as_str()));
    if file_path.exists() {
        println!("Citation file already found.");
        return;
    }
    if let Err(e) = fs::write(file_path, r.as_bytes()) {
        println!("Error writing citation file: {:?}", e);
    }
}
