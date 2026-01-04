use std::{fs, path::Path};

use super::Runner;

#[derive(Default)]
pub struct D03 {
    data: Vec<Box<str>>,
}

impl D03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for D03 {
    fn part1(&self) -> String {
        let mut sum: u32 = 0;

        for line in &self.data {
            let mut n1: u32 = 0;
            let mut n2: u32 = 0;
            for i in 0..line.len() {
                let num: u32 = line[i..=i].parse().unwrap();
                if num > n1 && (i + 1 != line.len()) {
                    n1 = num;
                    n2 = 0;
                    continue;
                }
                if num > n2 {
                    n2 = num;
                }
            }
            sum += n1 * 10 + n2;
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        "".into()
    }

    fn parse(&mut self) {
        let path = Path::new("./sols/day03.txt");
        let file = fs::read_to_string(path).unwrap();

        for l in file.lines() {
            self.data.push(l.into());
        }

        for x in &self.data {
            println!("{x}");
        }
    }
}
