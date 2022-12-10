use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map_res;
use nom::IResult;

use crate::util::Coordinates;
use crate::{DaySolution, FromInput};

pub struct Day9(Vec<Move>);

struct Move {
    direction: char,
    distance: usize,
}

impl Move {
    fn normal_vector(&self) -> Coordinates {
        match self.direction {
            'U' => Coordinates { x: 0, y: 1 },
            'D' => Coordinates { x: 0, y: -1 },
            'L' => Coordinates { x: -1, y: 0 },
            'R' => Coordinates { x: 1, y: 0 },
            other => panic!("Invalid direction {other}"),
        }
    }

    fn parse(input: &str) -> IResult<&str, Move> {
        let (input, direction) = anychar(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, distance) = map_res(digit1, |d: &str| d.parse())(input)?;

        Ok((
            input,
            Move {
                direction,
                distance,
            },
        ))
    }
}

struct Rope {
    segments: Vec<Coordinates>,
    tail_tracker: HashSet<Coordinates>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            segments: vec![Coordinates::default(); length],
            tail_tracker: HashSet::from([Coordinates::default()]),
        }
    }

    fn move_(&mut self, move_: &Move) {
        let normal_vector = move_.normal_vector();

        for _ in 0..move_.distance {
            self.segments[0] += normal_vector;

            for index in 0..(self.segments.len() - 1) {
                let first = &self.segments[index];
                let second = &self.segments[index + 1];

                let difference = *first - *second;
                if difference.x.abs() > 1 || difference.y.abs() > 1 {
                    self.segments[index + 1] += Coordinates {
                        x: difference.x.signum(),
                        y: difference.y.signum(),
                    };
                }
            }

            let tail = self.segments.last().expect("Rope cannot be empty");
            self.tail_tracker.insert(*tail);
        }
    }
}

impl FromInput for Day9 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, move_) = Move::parse(&line).expect("Invalid move");
                    move_
                })
                .collect(),
        )
    }
}

impl DaySolution for Day9 {
    fn part_one(&self) -> String {
        let mut rope = Rope::new(2);

        for move_ in &self.0 {
            rope.move_(move_);
        }

        rope.tail_tracker.len().to_string()
    }

    fn part_two(&self) -> String {
        let mut rope = Rope::new(10);

        for move_ in &self.0 {
            rope.move_(move_);
        }

        rope.tail_tracker.len().to_string()
    }
}
