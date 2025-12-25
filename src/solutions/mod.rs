mod d01;

pub use d01::D01;

pub trait Runner {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
    fn parse(&mut self);
}

fn output(mut sol: impl Runner) {
    sol.parse();
    println!("part1: {}", sol.part1());
    println!("part2: {}", sol.part2());
}

pub fn run(day: i32) {
    match day {
        1 => output(D01::new()),
        _ => println!("Day {day} not unimplemented"),
    }
}
