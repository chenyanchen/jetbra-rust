use anyhow::Result;
use clap::Parser;

use jetbra::{Args, Jetbra};

mod application;
mod install;
mod jetbra;
mod uninstall;

fn main() -> Result<()> {
    let args = Args::parse();
    Jetbra::new().run(args)
}
