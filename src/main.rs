use anyhow::Result;
use clap::Parser;

mod battery;

use crate::battery::cmd_battery;

/// Polybar custom modules
#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Parser)]
enum Cmd {
    Battery(BatteryArgs),
}

/// Custom battery module
#[derive(Debug, Parser)]
pub struct BatteryArgs {}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Cmd::Battery(args) => cmd_battery(args)?,
    }

    Ok(())
}
