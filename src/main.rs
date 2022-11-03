// #![deny(warnings)]

#![warn(unused_extern_crates)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]
// We allow expect since it forces good error messages at the least.
#![allow(clippy::expect_used)]

use tracing::debug;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use clap::Parser;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

include!("./cli_opt.rs");

mod rpmprovides;

fn main() {
    let fmt_layer = fmt::layer().with_writer(std::io::stderr);

    let filter_layer = match EnvFilter::try_from_default_env() {
        Ok(f) => f,
        Err(_) => EnvFilter::new("error"),
    };

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let opt = Opt::parse();
    debug!(?opt);

    match opt.command {
        Commands::RpmProvides => rpmprovides::exec(),
    }
}
