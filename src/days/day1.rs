use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::aoc::{AOCYearDay, Day};

type LocationIDsList = Vec<i32>;
type LocationIDsCounts = HashMap<i32, i32>;

#[derive(Debug, Clone)]
pub struct Day1 {
    list1: LocationIDsList,
    list2: LocationIDsList,
    // In order to reduce number of passes on second list we
    // collecting details on how many times we see a location ID
    // from second list in a table so that we can just look it up
    // for second part.
    counts: LocationIDsCounts,
}

impl Day for Day1 {
    type DayOutputPart1 = i32;
    type DayOutputPart2 = i32;

    fn id() -> AOCYearDay {
      (2024, 1)
    }

    fn part1(self) -> Result<Self::DayOutputPart1> {
        let mut l1 = self.list1.clone();
        let mut l2 = self.list2.clone();

        l1.sort();
        l2.sort();

        let distance =
            l1.iter()
                .zip(l2.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();

        Ok(distance)
    }

    fn part2(self) -> Result<Self::DayOutputPart2> {
        let similarity = self
            .list1
            .iter()
            .map(|a| a * self.counts.get(a).unwrap_or(&0))
            .sum();
        Ok(similarity)
    }
}

impl FromStr for Day1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list1: Vec<i32> = Vec::new();
        let mut list2: Vec<i32> = Vec::new();
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for line in s.split("\n") {
            if !line.trim().is_empty() {
                let entries: Vec<&str> = line.trim().split_whitespace().collect();
                if entries.len() != 2 {
                    return Err(anyhow!(
                        "Wrong input, got unexpected number of entries on a line: {}",
                        entries.len()
                    ));
                }
                let l1 = entries[0].parse::<i32>()?;
                list1.push(l1);
                let l2 = entries[1].parse::<i32>()?;
                list2.push(l2);
                let l2_counts = counts.entry(l2).or_insert(0);
                *l2_counts += 1;
            }
        }
        Ok(Day1 {
            list1,
            list2,
            counts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn day1_part1() -> Result<()> {
        let day = TEST_INPUT.parse::<Day1>()?;
        let result = day.part1()?;
        assert_eq!(result, 11);
        Ok(())
    }

    #[test]
    fn day1_part2() -> Result<()> {
        let day = TEST_INPUT.parse::<Day1>()?;
        let result = day.part2()?;
        assert_eq!(result, 31);
        Ok(())
    }
}
