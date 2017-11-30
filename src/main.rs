#[macro_use] extern crate failure_derive;
extern crate failure;
extern crate clap;

pub mod assets;
pub mod cargo;
pub mod error;
pub mod rustup;

use failure::Error;
use clap::{ App, Arg, SubCommand };
use cargo::*;
use rustup::setup;
use std::process::exit;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                         .version(env!("CARGO_PKG_VERSION"))
                         .author(env!("CARGO_PKG_AUTHORS"))
                         .about(env!("CARGO_PKG_DESCRIPTION"))
                         .subcommand(
                             SubCommand::with_name("wasm")
                                        .about("Commiting code and it's options")
                                        .subcommand(
                                            SubCommand::with_name("new")
                                                .about("Setup the environment for a wasm project")
                                                .arg(Arg::with_name("project_name")
                                                     .help("Name of the new wasm project")
                                                     .required(true)))
                                        .subcommand(
                                            SubCommand::with_name("build")
                                                .about("Build the wasm project")
                                                .arg(Arg::with_name("release")
                                                     .help("Make a release build")
                                                     .long("release")
                                                     .required(false)))
                                        .subcommand(
                                            SubCommand::with_name("run")
                                                .about("Run the wasm project")
                                                .arg(Arg::with_name("release")
                                                     .help("Run a release build")
                                                     .long("release")
                                                     .required(false))
                                                .arg(Arg::with_name("PATH")
                                                     .help("Path to directory to run")
                                                     .required(false))
                                                )
                                        .subcommand(
                                            SubCommand::with_name("setup")
                                                .about("Setup the environment for a wasm project"))
                        )
                         .get_matches();

    if let Some(matches) = matches.subcommand_matches("wasm") {

        if let Some(matches) = matches.subcommand_matches("new") {
            cargo_new(matches.value_of("project_name").unwrap())?;

        } else if let Some(matches) = matches.subcommand_matches("build") {
            let _release = matches.is_present("release");
            // There's a bug is wasm-gc isn't run
            cargo_build(true)?;

        } else if let Some(matches) = matches.subcommand_matches("run") {
            let _release = matches.is_present("release");
            // There's a bug is wasm-gc isn't run
            cargo_run(true, matches.value_of("PATH"))?;

        } else if let Some(_) = matches.subcommand_matches("setup") {
            setup()?;

        }
    }

    Ok(())
}
