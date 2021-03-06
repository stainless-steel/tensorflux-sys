extern crate pkg_config;

use std::{env, fs};
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const LIBRARY: &'static str = "tensorflow";
const REPOSITORY: &'static str = "https://github.com/tensorflow/tensorflow.git";
const TARGET: &'static str = "libtensorflow.so";
const VERSION: &'static str = "0.12";

macro_rules! get(($name:expr) => (ok!(env::var($name))));
macro_rules! ok(($expression:expr) => ($expression.unwrap()));

fn main() {
    if pkg_config::find_library(LIBRARY).is_ok() {
        return;
    }
    let output = PathBuf::from(&get!("OUT_DIR"));
    if !output.join(TARGET).exists() {
        let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("target/source");
        if !Path::new(&source.join(".git")).exists() {
            run("git", |command| command.arg("clone")
                                        .arg(format!("--branch=r{}", VERSION))
                                        .arg("--recursive")
                                        .arg(REPOSITORY)
                                        .arg(&source));
        }
        run_default(source.join("configure"), |command| command.current_dir(&source));
        run("bazel", |command| command.current_dir(&source)
                                      .arg("build")
                                      .arg(format!("--jobs={}", get!("NUM_JOBS")))
                                      .arg("--compilation_mode=opt")
                                      .arg(format!("{}:{}", LIBRARY, TARGET)));
        ok!(fs::copy(source.join("bazel-bin").join(LIBRARY).join(TARGET), output.join(TARGET)));
    }
    println!("cargo:rustc-link-lib=dylib={}", LIBRARY);
    println!("cargo:rustc-link-search={}", output.display());
}

fn run<S, F>(name: S, mut configure: F)
    where S: AsRef<OsStr>, F: FnMut(&mut Command) -> &mut Command
{
    let mut command = Command::new(name);
    let command = configure(&mut command);
    if !ok!(command.status()).success() {
        panic!("failed to execute {:?}", command);
    }
}

#[allow(unused_must_use)]
fn run_default<S, F>(name: S, mut configure: F)
    where S: AsRef<OsStr>, F: FnMut(&mut Command) -> &mut Command
{
    const PROMPT_COUNT: usize = 100;
    let mut command = Command::new(name);
    let command = configure(&mut command).stdin(Stdio::piped());
    let mut process = ok!(command.spawn());
    {
        let pipe = ok!(process.stdin.as_mut());
        for _ in 0..PROMPT_COUNT {
            pipe.write_all(b"\n");
        }
    }
    if !ok!(process.wait()).success() {
        panic!("failed to execute {:?}", command);
    }
}
