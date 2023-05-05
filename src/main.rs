use anyhow::Result;
use clap::Parser;

use jetbra::cmd::{Args, Jetbra};

fn main() -> Result<()> {
    let args = Args::parse();
    Jetbra::default().run(args)
}
