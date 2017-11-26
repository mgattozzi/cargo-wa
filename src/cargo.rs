//! Wrappers around cargo commands
use failure::Error;
use error::CargoWasmError::*;
use std::path::{ Path, PathBuf };
use std::process::{ Command, ExitStatus };
use std::fs;
use std::fs::{ OpenOptions, File };
use std::io::Write;
use assets::*;

pub fn cargo_build(release: bool) -> Result<(), Error> {
    if release {
        let exit = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--target=wasm32-unknown-unknown")
            .spawn()?
            .wait()?;
        if !exit.success() {
            return Err(CargoFail {
                exit: match exit_code(exit) {
                        Some(i) => format!("{}", i),
                        None => "No error code found".into(),
                      }
            }.into())
        }
    } else {
        let exit = Command::new("cargo")
            .arg("build")
            .arg("--target=wasm32-unknown-unknown")
            .spawn()?
            .wait()?;
        if !exit.success() {
            return Err(CargoFail {
                exit: match exit_code(exit) {
                        Some(i) => format!("{}", i),
                        None => "No error code found".into(),
                      }
            }.into())
        }
    }

    let wasm = find_wasm(release)?;

    // We know this is possible if we found the file and Rust should make it UTF-8 valid.
    // Hopefully. Should handle this better later.
    let filename = wasm.file_name().unwrap().to_str().unwrap();
    let path = format!("site/{}", &filename);
    fs::copy(&wasm, &path)?;

    if release {
        let temp = format!("{}/{}.temp", "site", &filename);
        let exit = Command::new("wasm-gc")
            .arg(&path)
            .arg(&temp)
            .spawn()?
            .wait()?;
        if !exit.success() {
            return Err(WasmGcFail {
                exit: match exit_code(exit) {
                        Some(i) => format!("{}", i),
                        None => "No error code found".into(),
                      }
            }.into())
        }
        fs::rename(temp, path)?;
    }

    Ok(())
}

// Assumes we're in the top level directory
fn find_wasm(release: bool) -> Result<PathBuf, Error> {
    let check_folder = if release {
        "target/wasm32-unknown-unknown/release"
    } else {
        "target/wasm32-unknown-unknown/debug"
    };
    for entry in fs::read_dir(check_folder)? {
        let entry = entry?;
        let path = entry.path();
        match path.extension() {
            Some(ext) => if ext == "wasm" {
                return Ok(entry.path());
            },
            None => {},
        }
    }

    Err(NoWasmCompiled.into())
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn exit_code(val: ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    match val.code() {
        Some(i) => Some(i),
        None => val.signal(),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn exit_code(val: ExitStatus) -> Option<i32> {
    val.code()
}

pub fn cargo_run(release: bool, path: Option<&str>) -> Result<(), Error> {
    cargo_build(release)?;

    match path {
        Some(p) => open_project(&Path::new(p)),
        None => open_project(&find_crate_root()?),
    }
}


pub fn cargo_new(project_name: &str) -> Result<(), Error> {
    let exit = Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(project_name)
        .spawn()?
        .wait()?;
    if !exit.success() {
        return Err(CargoFail {
            exit: match exit_code(exit) {
                    Some(i) => format!("{}", i),
                    None => "No error code found".into(),
                    }
        }.into())
    }

    fs::create_dir(format!("{}/site", project_name))?;
    let mut lib = File::create(format!("{}/src/lib.rs", project_name))?;
    let mut index = File::create(format!("{}/site/index.html", project_name))?;

    let index_string = String::from(INDEX_HTML);
    let index_html = index_string.replace("lib", project_name);

    lib.write_all(LIB_RS.as_bytes())?;
    index.write_all(index_html.as_bytes())?;

    let mut toml = OpenOptions::new().append(true).open(format!("{}/Cargo.toml", &project_name))?;
    const CRATE_TYPE: &str = "\n[lib]\ncrate-type = [\"cdylib\"]\n";
    toml.write_all(CRATE_TYPE.as_bytes())?;

    Ok(())
}

// TODO find the actual crate root
fn find_crate_root() -> Result<PathBuf, Error> {
    let mut path = PathBuf::new();
    path.push("site");
    path.push("index.html");
    Ok(path)
}

// Code below taken and modified from
// https://github.com/rust-lang/cargo/blob/61ca3022bca2c9e627d2ee523d7bd736e2bcaa2a/src/cargo/ops/cargo_doc.rs
// and is licensed under the terms of that repo
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn open_project(path: &Path) -> Result<(), Error> {
    use std::env;
    let mut methods = Vec::new();
    // trying $BROWSER
    if let Ok(name) = env::var("BROWSER") {
        match Command::new(name).arg(path).status() {
            Ok(_) => return Ok(()),
            Err(_) => methods.push("$BROWSER"),
        }
    }

    for m in ["xdg-open", "gnome-open", "kde-open"].iter() {
        match Command::new(m).arg(path).status() {
            Ok(_) => return Ok(()),
            Err(_) => methods.push(m),
        }
    }

    let mut method_folded = String::new();
    let len = methods.len();
    for (i, j) in methods.into_iter().enumerate() {
        method_folded.push_str(j);
        if i != len {
            method_folded.push(',');
        }
    }
    Err(BrowserOpenFail{ methods: method_folded }.into())
}

#[cfg(target_os = "windows")]
fn open_project(path: &Path) -> Result<(), Error> {
    match Command::new("cmd").arg("/C").arg(path).status() {
        Ok(_) => Ok(()),
        Err(_) => Err(BrowserOpenFail{ methods: String::from("cmd /C") }.into())
    }
}

#[cfg(target_os = "macos")]
fn open_project(path: &Path) -> Result<(), Error> {
    match Command::new("open").arg(path).status() {
        Ok(_) => Ok(()),
        Err(_) => Err(BrowserOpenFail{ methods: String::from("open") }.into())
    }
}

pub fn cargo_install_wasm_gc() -> Result<(), Error> {
    let exit = Command::new("cargo")
        .arg("install")
        .arg("--git")
        .arg("https://github.com/alexcrichton/wasm-gc")
        .arg("--force")
        .spawn()?
        .wait()?;
    if !exit.success() {
        return Err(CargoFail {
            exit: match exit_code(exit) {
                    Some(i) => format!("{}", i),
                    None => "No error code found".into(),
                    }
        }.into())
    }
    Ok(())
}
