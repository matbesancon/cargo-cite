# cargo-cite: a cargo extension to produce BibTeX from crates

## Why

Citing software is important to acknowledge the work of others,
but also because academic software development depends on it.  

One pain point developers have is to find **how** to cite a given library.
One has to look in the README, documentation or some other file.
A recent experiment in the Julia community is to standardize
citations in one file at the top-level of projects, named `CITATION.bib`
with all the relevant BibTeX entries for the project.
Multiple entries can be added for different sub-topics related to the
software, as you can see in the Julia [repo](https://github.com/JuliaLang/julia/blob/master/CITATION.bib).

## How does cargo-cite help

`cargo-cite` is an experimental Rust crate to generate a
`CITATION.bib` file for a Rust project based on its Cargo.toml file.
It can be yours or someone else's. Once the `CITATION.bib` file created,
feel free to add other entries to it, for example a software paper
published in the [Journal of Open-Source Software](http://joss.theoj.org).

## Usage

Say you are using [ndarray](https://github.com/rust-ndarray/ndarray.git)
for your work, but they have not published a CITATION.bib yet:

```
git clone https://github.com/rust-ndarray/ndarray.git
cd ndarray
cargo-cite
```

A `CITATION.bib` file has been created.

## Available options

See `cargo-cite --help` for options.
