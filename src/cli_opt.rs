use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[clap(about = "Modes that this command can run it")]
pub enum Commands {
    /// Generate a list of provides based on the built packages Cargo.lock.
    /// This is part of the internals of an rpm build and shouldn't be called
    /// directly. The "glue" is from /usr/lib/rpm/fileattrs/rust.attr which
    /// actually drives this command.
    RpmProvides,
}


#[derive(Debug, clap::Parser)]
#[clap(about = "Rust RPM Provides Helper")]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Commands,
}

