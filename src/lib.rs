#[macro_use]
extern crate error_chain;
extern crate clap;


// We'll put our errors in an `errors` module, and other modules in
// Create the Error, ErrorKind, ResultExt, and Result types
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
pub mod errors {
    error_chain!{}
}

pub mod helpers {
    use errors::*;
    use clap::ArgMatches;


    pub fn get_contacts() -> Result<()> {
        println!("get_contacts");
        use std::fs::File;
        // This operation will fail
        File::open("contacts")
            .chain_err(|| "unable to open ./contacts file")?;
        Ok(())
    }

    pub fn get_args(matches: &ArgMatches) -> Result<()> {
        println!("is foo present? {}", matches.is_present("foo"));

        if let Some(foo) = matches.value_of("foo") {
            println!("and has a value of {}", foo);
        } else {
            bail!("foo must have a value!");
        }

        println!("value of bar: {}", matches.value_of("bar").unwrap());

        Ok(())
    }
}