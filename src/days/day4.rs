use ndarray::Array2;
use std::str::FromStr;

use anyhow::Result;

use crate::aoc::{AOCYearDay, Day};

#[derive(Debug, Clone)]
pub struct Day4 {
    input: Array2<char>,
}

impl Day4 {
    fn check_xmas(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        if self.input[[x, y]] != 'X' {
            return 0;
        }
        let x_size = self.input.shape()[0];
        let y_size = self.input.shape()[1];

        // East direction
        if x < x_size - 3
            && self.input[[x + 1, y]] == 'M'
            && self.input[[x + 2, y]] == 'A'
            && self.input[[x + 3, y]] == 'S'
        {
            count += 1;
        }
        // West direction
        if x >= 3
            && self.input[[x - 1, y]] == 'M'
            && self.input[[x - 2, y]] == 'A'
            && self.input[[x - 3, y]] == 'S'
        {
            count += 1;
        }
        // South direction
        if y < y_size - 3
            && self.input[[x, y + 1]] == 'M'
            && self.input[[x, y + 2]] == 'A'
            && self.input[[x, y + 3]] == 'S'
        {
            count += 1;
        }
        // North direction
        if y >= 3
            && self.input[[x, y - 1]] == 'M'
            && self.input[[x, y - 2]] == 'A'
            && self.input[[x, y - 3]] == 'S'
        {
            count += 1;
        }
        // South-East direction
        if y < y_size - 3
            && x < x_size - 3
            && self.input[[x + 1, y + 1]] == 'M'
            && self.input[[x + 2, y + 2]] == 'A'
            && self.input[[x + 3, y + 3]] == 'S'
        {
            count += 1;
        }
        // South-West direction
        if y < y_size - 3
            && x >= 3
            && self.input[[x - 1, y + 1]] == 'M'
            && self.input[[x - 2, y + 2]] == 'A'
            && self.input[[x - 3, y + 3]] == 'S'
        {
            count += 1;
        }
        // North-East direction
        if y >= 3
            && x < x_size - 3
            && self.input[[x + 1, y - 1]] == 'M'
            && self.input[[x + 2, y - 2]] == 'A'
            && self.input[[x + 3, y - 3]] == 'S'
        {
            count += 1;
        }
        // North-West direction
        if y >= 3
            && x >= 3
            && self.input[[x - 1, y - 1]] == 'M'
            && self.input[[x - 2, y - 2]] == 'A'
            && self.input[[x - 3, y - 3]] == 'S'
        {
            count += 1;
        }
        count
    }

    fn check_x_mas(&self, x: usize, y: usize) -> bool {
        self.input[[x, y]] == 'A'
            && ((self.input[[x - 1, y - 1]] == 'M'
                && self.input[[x - 1, y + 1]] == 'M'
                && self.input[[x + 1, y - 1]] == 'S'
                && self.input[[x + 1, y + 1]] == 'S')
                || (self.input[[x - 1, y - 1]] == 'S'
                    && self.input[[x - 1, y + 1]] == 'M'
                    && self.input[[x + 1, y - 1]] == 'S'
                    && self.input[[x + 1, y + 1]] == 'M')
                || (self.input[[x - 1, y - 1]] == 'S'
                    && self.input[[x - 1, y + 1]] == 'S'
                    && self.input[[x + 1, y - 1]] == 'M'
                    && self.input[[x + 1, y + 1]] == 'M')
                || (self.input[[x - 1, y - 1]] == 'M'
                    && self.input[[x - 1, y + 1]] == 'S'
                    && self.input[[x + 1, y - 1]] == 'M'
                    && self.input[[x + 1, y + 1]] == 'S'))
    }
}

impl Day for Day4 {
    type DayOutputPart1 = usize;
    type DayOutputPart2 = usize;

    fn id() -> AOCYearDay {
        (2024, 4)
    }

    fn part1(self) -> Result<Self::DayOutputPart1> {
        let mut result = 0;
        let x_size = self.input.shape()[0];
        let y_size = self.input.shape()[1];

        for x in 0..x_size {
            for y in 0..y_size {
                result += self.check_xmas(x, y)
            }
        }
        Ok(result)
    }

    fn part2(self) -> Result<Self::DayOutputPart2> {
        let mut result = 0;
        let x_size = self.input.shape()[0];
        let y_size = self.input.shape()[1];

        for x in 1..(x_size - 1) {
            for y in 1..(y_size - 1) {
                if self.check_x_mas(x, y) {
                    result += 1
                }
            }
        }
        Ok(result)
    }
}

impl FromStr for Day4 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_size = 0;
        let mut y_size = 0;
        let data: Vec<char> = s
            .split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                x_size = l.len();
                y_size += 1;
                let chars: Vec<char> = l.chars().collect();
                chars
            })
            .flatten()
            .collect();
        let input = Array2::<char>::from_shape_vec((x_size, y_size), data)?;
        Ok(Day4 { input })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "\
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    #[test]
    fn day2_part1() -> Result<()> {
        let day = TEST_INPUT.parse::<Day4>()?;
        let result = day.part1()?;
        assert_eq!(result, 18);
        Ok(())
    }

    #[test]
    fn day2_part2() -> Result<()> {
        let day = TEST_INPUT.parse::<Day4>()?;
        let result = day.part2()?;
        assert_eq!(result, 9);
        Ok(())
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<Day4>().unwrap().part1().unwrap());
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| TEST_INPUT.parse::<Day4>().unwrap().part1().unwrap());
    }
}
