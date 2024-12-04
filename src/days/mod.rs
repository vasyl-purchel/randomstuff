use anyhow::Result;
use serde::Serialize;

use crate::aoc::process_day;

mod day1;
mod day2;
mod day3;
mod day4;

#[derive(
  clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Days {
  #[default]
  /// Day 1: Historian Hysteria
  Day1,
  /// Day 2: Red-Nosed Reports
  Day2,
  /// Day 3: Mull It Over
  Day3,
  /// Day 4: Ceres Search
  Day4,
}

pub fn solve(session_id: Option<String>, day: Days) -> Result<()> {
  match day {
    Days::Day1 => { let _ = process_day::<day1::Day1>(session_id)?; },
    Days::Day2 => { let _ = process_day::<day2::Day2>(session_id)?; },
    Days::Day3 => { let _ = process_day::<day3::Day3>(session_id)?; },
    Days::Day4 => { let _ = process_day::<day4::Day4>(session_id)?; },
  }
  Ok(())
}
