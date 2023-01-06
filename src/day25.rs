use std::fmt;

use crate::{DaySolution, FromInput};

pub struct Day25(Vec<Snafu>);

#[derive(Debug)]
pub struct Snafu {
    digits: Vec<isize>,
}

impl From<String> for Snafu {
    fn from(s: String) -> Snafu {
        Snafu {
            digits: s
                .chars()
                .map(|c| match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    other => panic!("Invalid SNAFU digit: {other}"),
                })
                .collect(),
        }
    }
}

impl From<&Snafu> for isize {
    fn from(snafu: &Snafu) -> isize {
        snafu
            .digits
            .iter()
            .fold(0, |product, digit| product * 5 + digit)
    }
}

impl From<isize> for Snafu {
    fn from(mut n: isize) -> Snafu {
        let mut digits = vec![];

        while n != 0 {
            let next_digit = (n + 2) % 5 - 2;
            digits.insert(0, next_digit);

            n = (n - next_digit) / 5;
        }

        Snafu { digits }
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in &self.digits {
            let c = match digit {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                other => panic!("Invalid SNAFU digit: {other}"),
            };

            write!(f, "{c}")?;
        }

        Ok(())
    }
}

impl FromInput for Day25 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(Into::into).collect())
    }
}

impl DaySolution for Day25 {
    fn part_one(&self) -> String {
        let sum = self.0.iter().map(|snafu| isize::from(snafu)).sum::<isize>();

        Snafu::from(sum).to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 25 using your parsed input")
    }
}
