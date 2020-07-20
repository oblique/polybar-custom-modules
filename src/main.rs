use anyhow::Result;
use argh::FromArgs;

mod battery;

use crate::battery::cmd_battery;

/// Polybar custom modules
#[derive(Debug, FromArgs)]
struct Args {
    #[argh(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Cmd {
    Battery(BatteryArgs),
}

/// Custom battery module
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "battery")]
pub struct BatteryArgs {}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    match args.cmd {
        Cmd::Battery(args) => cmd_battery(args)?,
    }

    Ok(())
}
