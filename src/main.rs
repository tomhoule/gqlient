#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate structopt;
extern crate graphql_parser;
#[macro_use]
extern crate quote;
extern crate serde;

use std::path::{Path, PathBuf};
use futures::prelude::*;

#[derive(StructOpt)]
struct Command {
    schema_url: Option<String>,
    schema_path: Option<PathBuf>,
    query_path: Option<PathBuf>,
}

fn gather_queries(schemas: Vec<(PathBuf, String)>, dir: &Path) {
    let root = ::std::env::current_dir().expect("current directory can be reached");
}

struct Query<Variables, Response> {
    response: ::std::marker::PhantomData<Response>,
    variables: Variables,
}

#[derive(Debug, Fail)]
enum QlientError {
    #[fail(display = "http error")]
    Http,
    #[fail(display = "serialization error")]
    Serialization,
}

impl<'response, Variables, Response> Query<Variables, Response>
where
    Response: serde::Deserialize<'response>,
    Variables: serde::Serialize
{
    fn new(variables: Variables) -> Self {
        Query {
            response: ::std::marker::PhantomData,
            variables,
        }
    }

    fn execute(self, backend_url: &str) -> impl Future<Item = Response, Error = QlientError> {
        unimplemented!()
    }
}

fn main() {
    let mut schemas = Vec::new();
    let current_dir = ::std::env::current_dir().expect("current directory is reachable");
    gather_queries(schemas, &current_dir);
}
