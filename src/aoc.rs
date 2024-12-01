use std::fmt::Display;
use std::fs;
use std::io;
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use reqwest::{blocking::ClientBuilder, cookie::Jar, Url};

pub type AOCYearDay = (i32, i32);

pub trait Day<T: FromStr + Sized = Self> {
    type DayOutputPart1: Display;
    type DayOutputPart2: Display;

    fn part1(self) -> Result<Self::DayOutputPart1>;
    fn part2(self) -> Result<Self::DayOutputPart2>;
    fn id() -> AOCYearDay;
}

pub enum DayPart {
    Part1,
    Part2,
}

fn parse<T: Day + FromStr>(file: &str) -> Result<T>
where
    <T as FromStr>::Err: Send + Sync + 'static,
    Result<T, <T as FromStr>::Err>: Context<T, <T as FromStr>::Err>,
{
    let time_tracker = Instant::now();
    let content = fs::read_to_string(file)?;
    let day = content
        .parse::<T>()
        .context(format!("can't parse '{}'", content))?;
    debug!("Parsing took: {}ms", time_tracker.elapsed().as_millis());
    Ok(day)
}

fn solve<T: Day + FromStr>(day: T, part: DayPart) -> Result<()> {
    let time_tracker = Instant::now();
    match part {
        DayPart::Part1 => {
            let result = day.part1()?;
            info!("Part 1 answer is: {}", result);
        }
        DayPart::Part2 => {
            let result = day.part2()?;
            info!("Part 2 answer is: {}", result);
        }
    }
    debug!("Parsing took: {}ms", time_tracker.elapsed().as_millis());
    Ok(())
}

pub fn process_day<T>(session_id: Option<String>) -> Result<()>
where
    T: Day + FromStr + Clone,
    <T as FromStr>::Err: Send + Sync + 'static,
    Result<T, <T as FromStr>::Err>: Context<T, <T as FromStr>::Err>,
{
    let (year, day) = <T>::id();
    let data_file = fetch_input(session_id, year, day)?;
    let day = parse::<T>(&data_file)?;
    let _ = solve(day.clone(), DayPart::Part1)?;
    let _ = solve(day, DayPart::Part2)?;
    Ok(())
}

pub fn fetch_input(session_id: Option<String>, year: i32, day: i32) -> Result<String> {
    let data_folder = format!("./data/{}", year);
    let data_file = format!("{}/{}.txt", data_folder, day);
    if fs::metadata(data_file.clone()).is_ok() {
        return Ok(data_file);
    }
    debug!("Ifnput ile {} is missing... downloading...", &data_file);
    let aoc_session_id = session_id.ok_or(anyhow!(
        "Please set aoc_session_id parameter to allow download input file"
    ))?;
    // ensure data folder is present
    fs::create_dir_all(data_folder)?;

    // create an input file
    let mut file = fs::File::create(data_file.clone())?;

    let url = "https://adventofcode.com".parse::<Url>()?;
    let cookie = format!("session={}", aoc_session_id);
    let cookie_jar = Jar::default();
    cookie_jar.add_cookie_str(&cookie, &url);
    let client = ClientBuilder::new()
        .cookie_provider(cookie_jar.into())
        .build()?;
    let mut resp = client
        .get(format!(
            "https://adventofcode.com/{}/day/{}/input",
            year, day
        ))
        .send()?;
    let _ = io::copy(&mut resp, &mut file)?;
    Ok(data_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[derive(Clone)]
    struct TestDay {
        foo: i32,
    }

    impl Day for TestDay {
        type DayOutputPart1 = i32;
        type DayOutputPart2 = i32;

        fn id() -> AOCYearDay {
            (0, 0)
        }

        fn part1(self) -> Result<Self::DayOutputPart1> {
            Ok(self.foo)
        }

        fn part2(self) -> Result<Self::DayOutputPart2> {
            Ok(self.foo)
        }
    }

    impl FromStr for TestDay {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let foo = s.parse::<i32>()?;
            Ok(TestDay { foo })
        }
    }

    #[test]
    fn test_parse_testday_bad_content() -> Result<()> {
        let day = parse::<TestDay>("./data/0/bad.txt");
        assert!(day.is_err());
        assert_eq!(day.err().unwrap().to_string(), "can't parse 'not a number'");
        Ok(())
    }

    #[test]
    fn test_process_testday_good() -> Result<()> {
        let _ = process_day::<TestDay>(None)?;
        Ok(())
    }
}
