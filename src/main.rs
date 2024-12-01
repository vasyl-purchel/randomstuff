use std::env;

use anyhow::Result;
use clap::Parser;

mod aoc;
mod days;

#[derive(Parser)]
#[command(styles = clap::builder::styling::Styles::styled())]
#[command(version, about, long_about = None)]
struct Cli {
    /// Advent of Code day to solve
    #[arg(short, long, default_value_t, value_enum)]
    day: days::Days,

    /// Advent of Code Session ID
    #[arg(long, env)]
    aoc_session_id: Option<String>,
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    let cli = Cli::parse();
    days::solve(cli.aoc_session_id, cli.day)?;
    Ok(())
}
