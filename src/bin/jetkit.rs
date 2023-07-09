use anyhow::Result;
use clap::{CommandFactory, Parser};

use jetbra::api::GetActiveCodeRequest;
use jetbra::install::Installer;
use jetbra::{api, hardware, install, jetbrains};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    Jetkit::new()?.run(args).await
}

struct Jetkit {
    installer: Installer,
}

impl Jetkit {
    fn new() -> Result<Self> {
        let installer = Installer::new(jetbrains::path()?);
        Ok(Self { installer })
    }

    async fn run(&self, args: Args) -> Result<()> {
        match args.token {
            Some(token) => self.install(token).await,
            None => Ok(Args::command().print_help()?),
        }
    }

    async fn install(&self, token: String) -> Result<()> {
        if token.len() != 22 {
            return Err(anyhow::anyhow!("Invalid token"));
        }
        // get active code
        let req = GetActiveCodeRequest::new(token, hardware::serial_number()?);
        let resp = api::get_active_code(&req).await?;
        // install
        self.installer.install(&install::Args { apps: resp.apps })
    }
}

// WVi9iGJvqskzUBwzmC4goT
// ZiDyvBmVjkaLCeBF9YkRBz

#[derive(Parser, Debug)]
#[command(author, version, long_about, verbatim_doc_comment)]
/// jetkit is a tool to active JetBrains products.
///
/// Steps to active:
///   1. Open JetBrains products.
///   2. Run `jetkit [TOKEN]` in terminal.
///   3. Restart JetBrains products.
struct Args {
    /// Token to active JetBrains products, get from c745539141@gamil.com.
    token: Option<String>,
}
