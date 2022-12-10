use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::{IResult, Parser};

use crate::{DaySolution, FromInput};

pub struct Day10(Vec<Instruction>);

enum Instruction {
    Noop,
    AddX(isize),
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        fn parse_addx(input: &str) -> IResult<&str, Instruction> {
            let (rest, _) = tag("addx ")(input)?;
            let amount = rest.parse().expect("Invalid amount");

            Ok(("", Instruction::AddX(amount)))
        }

        parse_addx
            .or(map(tag("noop"), |_| Instruction::Noop))
            .parse(input)
    }
}

impl FromInput for Day10 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| Instruction::parse(&line).expect("Invalid instruction").1)
                .collect(),
        )
    }
}

impl DaySolution for Day10 {
    fn part_one(&self) -> String {
        let observed_cycles: [isize; 6] = [20, 60, 100, 140, 180, 220];

        self.calculate_x_for_each_cycle()
            .filter_map(|(cycle, x)| {
                observed_cycles
                    .iter()
                    .find(|observed| cycle as isize == **observed - 1)
                    .map(|observed| x * *observed)
            })
            .sum::<isize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let chars = self
            .calculate_x_for_each_cycle()
            .take(240)
            .map(|(cycle, x)| {
                if ((cycle % 40) as isize - x).abs() <= 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .collect::<Vec<char>>();

        let x = chars
            .chunks(40)
            .map(|c| c.iter().collect())
            .collect::<Vec<String>>()
            .join("\n");

        format!("\n{x}\n")
    }
}

impl Day10 {
    fn calculate_x_for_each_cycle<'d>(&'d self) -> impl 'd + Iterator<Item = (usize, isize)> {
        self.0
            .iter()
            .scan(1isize, |x, instruction| match instruction {
                Instruction::Noop => Some(vec![*x]),
                Instruction::AddX(amount) => {
                    let old_x = *x;
                    *x += amount;
                    Some(vec![old_x, old_x])
                }
            })
            .flat_map(|xs| xs)
            .enumerate()
    }
}
