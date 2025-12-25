use std::{fs, path::Path};

use super::Runner;

enum Turn {
    R(i32),
    L(i32),
}

#[derive(Default)]
pub struct D01 {
    data: Vec<Turn>,
}

impl D01 {
    pub fn new() -> Self {
        Self::default()
    }
}

const MIN: i32 = 0;
const MAX: i32 = 99;
const MODULUS: i32 = MAX - MIN + 1;

impl Runner for D01 {
    fn part1(&self) -> String {
        let mut dial = 50;
        let mut counter = 0;

        for j in &self.data {
            match j {
                Turn::R(x) => dial += x,
                Turn::L(x) => dial -= x,
            }

            dial = (dial).rem_euclid(MODULUS);

            if dial == 0 {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn part2(&self) -> String {
        let mut dial = 50;
        let mut counter = 0;

        for j in &self.data {
            match j {
                Turn::R(x) => {
                    if dial + x >= MODULUS {
                        counter += (x + dial) / MODULUS;
                    }
                    dial = (dial + x).rem_euclid(MODULUS);
                }
                Turn::L(x) => {
                    if dial - *x <= MIN {
                        counter += ((*x - dial) / MODULUS) + if dial == 0 { 0 } else { 1 };
                    }

                    dial = (dial - *x).rem_euclid(MODULUS);
                }
            }
        }
        counter.to_string()
    }

    fn parse(&mut self) {
        let path = Path::new("./sols/day01.txt");
        let file = fs::read_to_string(path).unwrap();

        for l in file.lines() {
            let direction = &l[..1];
            let turn = &l[1..];

            self.data.push(match direction {
                "R" => Turn::R(turn.parse::<i32>().unwrap()),
                "L" => Turn::L(turn.parse::<i32>().unwrap()),
                _ => Turn::R(0),
            });
        }
    }
}
