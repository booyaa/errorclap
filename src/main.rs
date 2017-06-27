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
use clap::{Arg, App, SubCommand};


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
                          .subcommand(SubCommand::with_name("contacts")
                                      .about("loads contacts"))
                            .subcommand(SubCommand::with_name("args")
                                      .about("check args"))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("contacts") {
        // adds error to the list of errors
        get_contacts().chain_err(|| "failed to get contacts")?;
    }


    // loses inner error
    // if get_contacts().is_err() {
    //     bail!("error!");
    // }

    get_args().chain_err(|| "failed to get args")?;

    Ok(())
}



fn get_contacts() -> Result<()> {
    println!("get_contacts");
    use std::fs::File;
    // This operation will fail
    File::open("contacts").chain_err(|| "unable to open ./contacts file")?;
    Ok(())
}

fn get_args() -> Result<()> {
    println!("get_args");
    use std::env;

    if env::args_os().count() < 2 {
        bail!("no args passed, usage: errorclap foo bar"); // early exit
    }

    for args in env::args_os() {
        println!("\t{:?}", args);
    }

    Ok(())
}