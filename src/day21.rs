use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::tuple;
use nom::IResult;

use crate::{DaySolution, FromInput};

const ROOT_NAME: &'static str = "root";
const HUMAN_NAME: &'static str = "humn";

// TODO: Model the problem into this struct
#[derive(Debug)]
pub struct Day21(HashMap<String, MonkeyJob>);

#[derive(Debug, Clone)]
enum MonkeyJob {
    Number(isize),
    Combine(CombineJob),
}

impl MonkeyJob {
    fn parse(input: &str) -> IResult<&str, (String, Self)> {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;

        let (input, job) = alt((
            map(
                tuple((alpha1, tag(" "), Operator::parse, tag(" "), alpha1)),
                |(left, _, operator, _, right)| {
                    Self::Combine(CombineJob {
                        left: left.to_owned(),
                        right: right.to_owned(),
                        operator,
                    })
                },
            ),
            map(parse_isize, MonkeyJob::Number),
        ))(input)?;

        Ok((input, (name.to_owned(), job)))
    }
}

#[derive(Debug, Clone)]
struct CombineJob {
    left: String,
    right: String,
    operator: Operator,
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |d: &str| d.parse())(input)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("+"), |_o| Operator::Add),
            map(tag("-"), |_o| Operator::Subtract),
            map(tag("*"), |_o| Operator::Multiply),
            map(tag("/"), |_o| Operator::Divide),
        ))(input)
    }

    fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Operator::Add => left + right,
            Operator::Subtract => left - right,
            Operator::Multiply => left * right,
            Operator::Divide => left / right,
        }
    }
}

impl FromInput for Day21 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, (name, job)) = MonkeyJob::parse(&line).expect("Invalid monkey job");
                    (name, job)
                })
                .collect(),
        )
    }
}

impl DaySolution for Day21 {
    fn part_one(&self) -> String {
        let (results, work_queue) = self.separate_results_and_work();
        let final_results = find_all_results(results, work_queue);

        final_results[ROOT_NAME].to_string()
    }

    fn part_two(&self) -> String {
        let (results, mut work_queue) = self.separate_results_and_work();
        let (_name, _root_combination) = work_queue
            .iter_mut()
            .find(|(name, _combine)| name == &&ROOT_NAME)
            .unwrap();

        for human_value in -1_000..1_000 {
            let mut results = results.clone();
            results.insert(HUMAN_NAME, human_value);

            let final_results = find_all_results(results, work_queue.clone());

            if final_results[ROOT_NAME] == 1 {
                return human_value.to_string();
            }
        }

        "no answer found".to_owned()
    }
}

impl Day21 {
    fn separate_results_and_work(&self) -> (HashMap<&str, isize>, HashMap<&str, &CombineJob>) {
        let results: HashMap<&str, isize> = self
            .0
            .iter()
            .filter_map(|(name, job)| match job {
                MonkeyJob::Number(num) => Some((name.as_str(), *num)),
                MonkeyJob::Combine(_) => None,
            })
            .collect();
        let work_queue = self
            .0
            .iter()
            .filter_map(|(name, job)| match job {
                MonkeyJob::Combine(combine) => Some((name.as_str(), combine)),
                MonkeyJob::Number(_) => None,
            })
            .collect::<HashMap<&str, &CombineJob>>();

        (results, work_queue)
    }
}

fn find_all_results<'d>(
    mut results: HashMap<&'d str, isize>,
    mut work_queue: HashMap<&'d str, &'d CombineJob>,
) -> HashMap<&'d str, isize> {
    while !work_queue.is_empty() {
        let (name, result) = work_queue
            .iter()
            .find_map(|(name, combine)| {
                match (
                    results.get(combine.left.as_str()),
                    results.get(combine.right.as_str()),
                ) {
                    (Some(left), Some(right)) => {
                        Some((name.to_owned(), combine.operator.apply(*left, *right)))
                    }
                    (_, _) => None,
                }
            })
            .expect("Unable to reduce jobs");

        work_queue.remove(name);
        results.insert(name, result);
    }

    results
}

// struct Polynomial {
//     coefficients: HashMap<isize, Fraction>,
// }

// struct Fraction {
//     numerator: isize,
//     denominator: isize,
// }

// impl Add for Fraction {
//     type Output = Self;

//     fn add(self, other: Self) -> Self::Output {
//         Self {
//             numerator: self.numerator * other.denominator + other.numerator * self.denominator,
//             denominator: self.denominator * other.denominator,
//         }
//     }
// }

// impl Mul for Fraction {

// }
