use anyhow::Result;
use clap::Parser;

use jetbra::jetbra::{Args, Jetbra};

fn main() -> Result<()> {
    let args = Args::parse();
    Jetbra::default().run(args)
}
