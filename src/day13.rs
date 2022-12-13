use std::cmp::Ordering;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::{DaySolution, FromInput};

pub struct Day13(Vec<(Packet, Packet)>);

#[derive(PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(usize),
}

impl Packet {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_int = map_res(digit1, |n: &str| n.parse()).map(Packet::Int);

        delimited(
            tag("["),
            separated_list0(tag(","), Self::parse.or(parse_int)).map(Self::List),
            tag("]"),
        )(input)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(ls), Packet::List(rs)) => lists_in_correct_order(ls.iter(), rs.iter()),
            (l @ Packet::Int(_), Packet::List(rs)) => {
                lists_in_correct_order(Some(l).into_iter(), rs.iter())
            }
            (Packet::List(ls), r @ Packet::Int(_)) => {
                lists_in_correct_order(ls.iter(), Some(r).into_iter())
            }
        }
    }
}

fn lists_in_correct_order<'p>(
    mut left: impl Iterator<Item = &'p Packet>,
    mut right: impl Iterator<Item = &'p Packet>,
) -> Ordering {
    loop {
        match (left.next(), right.next()) {
            (Some(l), Some(r)) => match l.cmp(r) {
                Ordering::Equal => continue,
                not_equal => return not_equal,
            },
            (Some(_l), None) => return Ordering::Greater,
            (None, Some(_r)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromInput for Day13 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .chain(Some("".to_owned()))
                .array_chunks()
                .map(|[left, right, _empty]| {
                    let (_rest, left) = Packet::parse(&left).expect("Invalid left packet");
                    let (_rest, right) = Packet::parse(&right).expect("Invalid right packet");

                    (left, right)
                })
                .collect(),
        )
    }
}

impl DaySolution for Day13 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .enumerate()
            .filter_map(
                |(index, (left, right))| {
                    if left < right {
                        Some(index + 1)
                    } else {
                        None
                    }
                },
            )
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let decoders = [
            Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
            Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
        ];

        let mut packets: Vec<&Packet> = self
            .0
            .iter()
            .flat_map(|(left, right)| [left, right])
            .chain(&decoders)
            .collect();
        packets.sort();

        decoders
            .iter()
            .map(|d| {
                let index = packets
                    .iter()
                    .position(|p| p == &d)
                    .expect("Cannot find decoder");
                index + 1
            })
            .product::<usize>()
            .to_string()
    }
}
