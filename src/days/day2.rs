use std::str::FromStr;

use anyhow::Result;
use log::trace;

use crate::aoc::{AOCYearDay, Day};

type Report = Vec<i32>;

#[derive(Debug, Clone)]
pub struct Day2 {
    reports: Vec<Report>,
}

impl Day2 {
    const UPPER_LIMIT: i32 = 3;
    const LOWER_LIMIT: i32 = 1;

    fn in_limits(a: i32, b: i32) -> bool {
        let x = (a - b).abs();
        (Day2::LOWER_LIMIT <= x) && (x <= Day2::UPPER_LIMIT)
    }

    fn check_report_step_validity(a: i32, b: i32, c: i32) -> bool {
        if ((a > b) && (b > c)) || ((c > b) && (b > a)) {
            // 3 entries are homogeniously increasing/decreasing
            if Day2::in_limits(a, b) && Day2::in_limits(b, c) {
                // all entries are between the limits
                return true;
            }
        }
        false
    }

    fn damp(report: &Report, position: usize) -> Report {
        trace!("Damping position {} for rerport <{:?}>", position, report);
        let mut dampped_report = report.clone();
        dampped_report.remove(position);
        dampped_report
    }

    fn check_report_safe(report: &Report, damp_enabled: bool) -> bool {
        if report.len() < 2 {
            return true;
        }

        for (position, entry) in report.iter().enumerate() {
            match position {
                0 => {
                    continue;
                }
                1 => {
                    if !Day2::in_limits(report[0], *entry) {
                        trace!("Step {}:[1]{} failed\t<{:?}>", report[0], entry, report);
                        return damp_enabled
                            && (Day2::check_report_safe(&Day2::damp(report, 0), false)
                                || Day2::check_report_safe(&Day2::damp(report, 1), false));
                    }
                }
                p => {
                    if !Day2::check_report_step_validity(report[p - 2], report[p - 1], *entry) {
                        trace!(
                            "Step {}:{}:[{}]{} failed\t<{:?}>",
                            report[p - 2],
                            report[p - 1],
                            p,
                            entry,
                            report
                        );
                        return damp_enabled
                            && (Day2::check_report_safe(&Day2::damp(report, p - 2), false)
                                || Day2::check_report_safe(&Day2::damp(report, p - 1), false)
                                || Day2::check_report_safe(&Day2::damp(report, p), false));
                    }
                }
            }
        }
        trace!("Report is safe: <{:?}>", report);
        true
    }
}

impl Day for Day2 {
    type DayOutputPart1 = usize;
    type DayOutputPart2 = usize;

    fn id() -> AOCYearDay {
        (2024, 2)
    }

    fn part1(self) -> Result<Self::DayOutputPart1> {
        let safe_reports = self
            .reports
            .iter()
            .map(|r| Day2::check_report_safe(r, false))
            .filter(|it| *it)
            .count();

        Ok(safe_reports)
    }

    fn part2(self) -> Result<Self::DayOutputPart2> {
        let safe_reports = self
            .reports
            .iter()
            .map(|r| Day2::check_report_safe(r, true))
            .filter(|it| *it)
            .count();

        Ok(safe_reports)
    }
}

impl FromStr for Day2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reports = Vec::<Report>::new();
        for line in s.split("\n") {
            let mut report = Report::new();
            for entry in line.trim().split_whitespace() {
                let val = entry.parse::<i32>()?;
                report.push(val);
            }
            if report.len() > 0 {
                reports.push(report);
            }
        }
        Ok(Day2 { reports })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "\
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn day2_part1() -> Result<()> {
        let day = TEST_INPUT.parse::<Day2>()?;
        let result = day.part1()?;
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn day2_part2() -> Result<()> {
        let day = TEST_INPUT.parse::<Day2>()?;
        let result = day.part2()?;
        assert_eq!(result, 4);
        Ok(())
    }

    // adds '4 7 5 3 1' entry where failure occurs on 7-5 check
    // but removal is needed of '4' instead
    const SECOND_TEST_INPUT: &str = "\
        5 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 6
        4 7 5 3 1
    ";

    #[test]
    fn day2_part1_test2() -> Result<()> {
        let day = SECOND_TEST_INPUT.parse::<Day2>()?;
        let result = day.part1()?;
        assert_eq!(result, 0);
        Ok(())
    }

    #[test]
    fn day2_part2_test2() -> Result<()> {
        let day = SECOND_TEST_INPUT.parse::<Day2>()?;
        let result = day.part2()?;
        assert_eq!(result, 5);
        Ok(())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<Day2>().unwrap().part1().unwrap());
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<Day2>().unwrap().part1().unwrap());
    }
}
