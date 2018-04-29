#[macro_use]
extern crate structopt;
extern crate graphql_parser;
#[macro_use]
extern crate quote;

use std::path::{Path, PathBuf};

#[derive(StructOpt)]
struct Command {
    schema_url: Option<String>,
    schema_path: Option<PathBuf>,
    query_path: Option<PathBuf>,
}

fn gather_queries(schemas: Vec<(PathBuf, String)>, dir: &Path) {
    let root = ::std::env::current_dir().expect("current directory can be reached");
}

fn main() {
    let mut schemas = Vec::new();
    let current_dir = ::std::env::current_dir().expect("current directory is reachable");
    gather_queries(schemas, &current_dir);
}
