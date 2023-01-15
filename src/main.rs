mod app;
mod map;
mod puget_sound;
mod ui;

#[cfg(feature = "crossterm")]
mod crossterm;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;

extern crate reqwest;

use argh::FromArgs;
use std::{error::Error, time::Duration};

/// Ferris Main app
#[derive(Debug, FromArgs)]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "5000")]
    tick_rate: u64,
    /// whether unicode symbols are used to improve the overall look of the app
    #[argh(option, default = "true")]
    enhanced_graphics: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics).await
}


