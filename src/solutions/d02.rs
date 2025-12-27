use std::{fs, path::Path};

use super::Runner;

struct Range {
    start: u64,
    end: u64,
}

#[derive(Default)]
pub struct D02 {
    data: Vec<Range>,
}

impl D02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for D02 {
    fn part1(&self) -> String {
        let mut sum: u64 = 0;
        for r in &self.data {
            for i in r.start..=r.end {
                let num = i.to_string();
                if num.len() % 2 != 0 {
                    continue;
                }

                let mid = num.len() / 2;
                if num[0..mid] == num[mid..] {
                    sum += num.parse::<u64>().unwrap();
                }
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        "".into()
    }

    fn parse(&mut self) {
        let path = Path::new("./sols/day02.txt");
        let file = fs::read_to_string(path).unwrap();

        let raw = file.lines().next().unwrap();
        for r in raw.split(",") {
            let tmp: Vec<&str> = r.split("-").collect();
            self.data.push(Range {
                start: tmp[0].parse::<u64>().unwrap(),
                end: tmp[1].parse::<u64>().unwrap(),
            });
        }
    }
}
