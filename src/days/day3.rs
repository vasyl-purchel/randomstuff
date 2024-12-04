use regex::Regex;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::aoc::{AOCYearDay, Day};

#[derive(Debug, Clone)]
enum Operation {
    MUL(i32, i32),
    DO,
    DONT,
}

#[derive(Debug, Clone)]
pub struct Day3 {
    operations: Vec<Operation>,
}

impl Day for Day3 {
    type DayOutputPart1 = i32;
    type DayOutputPart2 = i32;

    fn id() -> AOCYearDay {
        (2024, 3)
    }

    fn part1(self) -> Result<Self::DayOutputPart1> {
        Ok(self
            .operations
            .iter()
            .map(|op| {
                if let Operation::MUL(a, b) = op {
                    a * b
                } else {
                    0
                }
            })
            .sum())
    }

    fn part2(self) -> Result<Self::DayOutputPart2> {
        let result = self
            .operations
            .iter()
            .fold((0, true), |(sum, process), op| match op {
                Operation::MUL(a, b) => {
                    if process {
                        (sum + a * b, process)
                    } else {
                        (sum, process)
                    }
                }
                Operation::DO => (sum, true),
                Operation::DONT => (sum, false),
            });
        Ok(result.0)
    }
}

impl FromStr for Day3 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut operations = Vec::<Operation>::new();
        let re = Regex::new(r"(mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)|do\(\)|don't\(\))")?;
        for x in re.find_iter(s) {
            if x.as_str().starts_with("mul") {
                if let Some(numbers) = re.captures(x.as_str()) {
                    let a = numbers["a"].to_string().parse::<i32>()?;
                    let b = numbers["b"].to_string().parse::<i32>()?;
                    operations.push(Operation::MUL(a, b));
                }
            } else if x.as_str() == "do()" {
                operations.push(Operation::DO);
            } else if x.as_str() == "don't()" {
                operations.push(Operation::DONT)
            } else {
                return Err(anyhow!("Unexpected match: {:?}", x));
            }
        }
        Ok(Day3 { operations })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn day2_part1() -> Result<()> {
        let day = TEST_INPUT.parse::<Day3>()?;
        let result = day.part1()?;
        assert_eq!(result, 161);
        Ok(())
    }

    #[test]
    fn day2_part2() -> Result<()> {
        let day = TEST_INPUT_2.parse::<Day3>()?;
        let result = day.part2()?;
        assert_eq!(result, 48);
        Ok(())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<Day3>().unwrap().part1().unwrap());
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| TEST_INPUT_2.parse::<Day3>().unwrap().part1().unwrap());
    }
}
