#![allow(dead_code)]

use std::env;
use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

include!("src/cli_opt.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    let comp_dir = PathBuf::from(outdir)
        .ancestors()
        .nth(2)
        .map(|p| p.join("completions"))
        .expect("Unable to process completions path");

    if !comp_dir.exists() {
        std::fs::create_dir(&comp_dir).expect("Unable to create completions dir");
    }

    generate_to(
        Shell::Bash,
        &mut Opt::command(),
        "rust-rpm-prov",
        comp_dir.clone(),
    )
    .ok();
    generate_to(
        Shell::Zsh,
        &mut Opt::command(),
        "rust-rpm-prov",
        comp_dir.clone(),
    )
    .ok();
}
