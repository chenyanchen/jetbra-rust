use anyhow::Result;
use clap::Parser;

use jetbra::cmd::{Jetbra, JetbraArgs};

fn main() -> Result<()> {
    let args = JetbraArgs::parse();
    Jetbra::default().run(args)
}
