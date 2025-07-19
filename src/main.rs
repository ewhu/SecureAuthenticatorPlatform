// src/main.rs
/*
 * Main executable for SecureAuthenticatorPlatform
 */

use clap::Parser;
use secureauthenticatorplatform::{Result, run};

#[derive(Parser)]
#[command(version, about = SecureAuthenticatorPlatform - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
