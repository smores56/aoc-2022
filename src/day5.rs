use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

use crate::{DaySolution, FromInput};

pub struct Day5 {
    crate_stacks: Vec<Vec<char>>,
    orders: Vec<MoveOrder>,
}

pub struct MoveOrder {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_crate_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(
        tag(" "),
        map(delimited(tag("["), anychar, tag("]")), Some).or(map(tag("   "), |_t: &str| None)),
    )(input)
}

fn parse_order(input: &str) -> IResult<&str, MoveOrder> {
    let (input, _) = tag("move ")(input)?;
    let (input, amount) = parse_usize(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = parse_usize(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = parse_usize(input)?;

    Ok((input, MoveOrder { amount, from, to }))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |num: &str| num.parse())(input)
}

impl FromInput for Day5 {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        let mut crate_stacks = vec![];

        while let Ok((_rest, crate_row)) = parse_crate_row(&lines.next().unwrap()) {
            if crate_stacks.is_empty() {
                crate_stacks = std::iter::repeat(vec![]).take(crate_row.len()).collect();
            }

            for (index, crate_) in crate_row.into_iter().enumerate() {
                if let Some(crate_) = crate_ {
                    crate_stacks[index].push(crate_);
                }
            }
        }

        let orders = lines
            .skip(1)
            .map(|line| {
                let (_rest, order) = parse_order(&line).unwrap();
                order
            })
            .collect();

        Self {
            crate_stacks,
            orders,
        }
    }
}

impl DaySolution for Day5 {
    fn part_one(&self) -> String {
        let mut stacks = self.crate_stacks.clone();

        for order in &self.orders {
            for _ in 0..order.amount {
                let popped = stacks[order.from - 1].remove(0);
                stacks[order.to - 1].insert(0, popped);
            }
        }

        stacks.iter().map(|stack| stack[0]).collect()
    }

    fn part_two(&self) -> String {
        let mut stacks = self.crate_stacks.clone();

        for order in &self.orders {
            let mut moving_stack: Vec<char> =
                stacks[order.from - 1].drain(0..order.amount).collect();
            moving_stack.append(&mut stacks[order.to - 1]);

            stacks[order.to - 1] = moving_stack;
        }

        stacks.iter().map(|stack| stack[0]).collect()
    }
}
