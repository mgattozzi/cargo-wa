//! Commands for Rustup based actions

use failure::Error;
use error::CargoWasmError::*;
use cargo::{ cargo_install_wasm_gc, exit_code };
use std::process::Command;

/// Sets up the environment to work with wasm
pub fn setup() -> Result<(), Error> {
    let exit = Command::new("rustup")
            .arg("update")
            .arg("nightly")
            .spawn()?
            .wait()?;
    if !exit.success() {
        return Err(RustupFail {
            exit: match exit_code(exit) {
                    Some(i) => format!("{}", i),
                    None => "No error code found".into(),
                    }
        }.into())
    }

    let exit = Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg("wasm32-unknown-unknown")
            .arg("--toolchain")
            .arg("nightly")
            .spawn()?
            .wait()?;
    if !exit.success() {
        return Err(RustupFail {
            exit: match exit_code(exit) {
                    Some(i) => format!("{}", i),
                    None => "No error code found".into(),
                    }
        }.into())
    }

    cargo_install_wasm_gc()?;
    Ok(())
}
