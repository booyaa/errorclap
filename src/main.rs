// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;
extern crate clap;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}


use errors::*;
use clap::{Arg, App, SubCommand, ArgMatches};


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
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("contacts").about("loads contacts"))
        .subcommand(SubCommand::with_name("args")
                            .about("check args")
                            .arg(Arg::with_name("foo")
                                .short("f")
                                .long("foo")                                
                                .value_name("FOO")
                                .required(true))
                            .arg(Arg::with_name("bar")
                                .short("b")
                                .long("bar")
                                .default_value("bar")
                                .value_name("BAR"))
                                )
        .get_matches();

    match matches.subcommand() {
        ("contacts", _) => get_contacts().chain_err(|| "failed to get contacts!"),
        ("args", Some(m)) => get_args(m).chain_err(|| "failed to parse args!"),
        _ => Ok(()),
    }
}



fn get_contacts() -> Result<()> {
    println!("get_contacts");
    use std::fs::File;
    // This operation will fail
    File::open("contacts").chain_err(|| "unable to open ./contacts file")?;
    Ok(())
}

fn get_args(matches: &ArgMatches) -> Result<()> {
    println!("is foo present? {}", matches.is_present("foo"));

    if let Some(foo) = matches.value_of("foo") {
        println!("and has a value of {}", foo);
    } else {
        bail!("foo must have a value!");
    }

    println!("value of bar: {}", matches.value_of("bar").unwrap());

    Ok(())
}