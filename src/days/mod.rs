use anyhow::Result;
use serde::Serialize;

use crate::aoc::process_day;

mod day1;

#[derive(
  clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Days {
  #[default]
  /// Day 1: Historian Hysteria
  Day1,
}

pub fn solve(session_id: Option<String>, day: Days) -> Result<()> {
  match day {
    Days::Day1 => { let _ = process_day::<day1::Day1>(session_id)?; },
  }
  Ok(())
}
