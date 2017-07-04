// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
extern crate error_chain;
extern crate clap;

extern crate errorclap;

use clap::{Arg, App, SubCommand};

use errorclap::errors::*;
use errorclap::helpers;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    setup()
}

fn setup() -> Result<()> {
    let matches = App::new("👏 clap yo error chains ⛓")
        .version("1.0")
        .author("booyaa <email@example.com>")
        .about(
            "Robert'); DROP TABLE STUDENTS; -- seriously you don't clean you input?",
        )
        .subcommand(SubCommand::with_name("contacts").about("loads contacts"))
        .subcommand(
            SubCommand::with_name("args")
                .about("check args")
                .arg(
                    Arg::with_name("foo")
                        .short("f")
                        .long("foo")
                        .value_name("FOO")
                        .required(true),
                )
                .arg(
                    Arg::with_name("bar")
                        .short("b")
                        .long("bar")
                        .default_value("bar")
                        .value_name("BAR"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("contacts", _) => helpers::get_contacts().chain_err(|| "failed to get contacts!"),
        ("args", Some(m)) => helpers::get_args(m).chain_err(|| "failed to parse args!"),
        _ => Ok(()),
    }
}