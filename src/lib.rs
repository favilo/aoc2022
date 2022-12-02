// #![feature(box_patterns, box_syntax)]
// #![feature(box_syntax)]

use std::{
    fmt::Debug,
    fs::read_to_string,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::Result;

use crate::utils::download_input;

mod utils;

const YEAR: usize = 2022;

macro_rules! run_days {
    ($day:ident = $id:expr, $($days:ident = $ids:expr),* $(,)?) => {
        pub mod $day;
        $(pub mod $days;)*
        pub fn run(days: Vec<usize>) -> Result<Duration> {
            let mut total_time = Duration::ZERO;
            if days.is_empty() {
                total_time += $day::Day::run()?;
                $(total_time += $days::Day::run()?;)+
            } else {
                for day in days {
                    total_time += match day {
                        $id => $day::Day::run()?,
                        $($ids => $days::Day::run()?,)*
                        _ => panic!("Invalid day passed"),
                    }
                }
            }

            Ok(total_time)
        }
    };
}

run_days!(
    day01 = 1,
    day02 = 2,
);

pub trait Runner<Part1 = usize, Part2 = usize>
where
    Part1: Debug,
    Part2: Debug,
{
    type Input;

    fn run() -> Result<Duration> {
        let comment = Self::comment();
        let comment = if comment.is_empty() {
            comment.to_owned()
        } else {
            format!(" : {}", comment)
        };
        log::info!("Day {}{}\n", Self::day(), comment);
        let input_path = format!("input/{}/day{:02}.txt", YEAR, Self::day());
        if !Path::new(&input_path).exists() {
            let session_file = home::home_dir().expect("home exists").join(".aocsession");
            let session = read_to_string(&session_file).expect("reading .aocsession file");
            download_input(Self::day(), YEAR, &session, &input_path)?;
        }
        let input = read_to_string(input_path)?;
        let now = Instant::now();
        let input = Self::get_input(&input)?;
        let elapsed_i = now.elapsed();
        log::info!("Generation took {:?}", elapsed_i);

        let now = Instant::now();
        let output1 = Self::part1(&input);
        let elapsed1 = now.elapsed();
        let output1 = output1?;
        log::info!("Part 1 - {:?}", output1);
        log::info!("Took {:?}", elapsed1);

        let now = Instant::now();
        let output2 = Self::part2(&input);
        let elapsed2 = now.elapsed();
        let output2 = output2?;

        log::info!("Part 2 - {:?}", output2);
        log::info!("Took {:?}\n", elapsed2);
        Ok(elapsed_i + elapsed1 + elapsed2)
    }

    fn day() -> usize;
    fn comment() -> &'static str {
        ""
    }

    fn get_input(_: &str) -> Result<Self::Input>;
    fn part1(_: &Self::Input) -> Result<Part1>;
    fn part2(_: &Self::Input) -> Result<Part2>;
}
