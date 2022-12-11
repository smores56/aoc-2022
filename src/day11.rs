use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::{IResult, Parser};

use crate::{DaySolution, FromInput};

#[derive(Clone, Debug)]
struct Monkey {
    worries: Vec<usize>,
    operation: Operation,
    test_modulo: usize,
    test_true_target: usize,
    test_false_target: usize,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Monkey ")(input)?;
        let (input, _index) = parse_usize(input)?;
        let (input, _) = tag(":\n")(input)?;

        let (input, _) = tag("  Starting items: ")(input)?;
        let (input, worries) = separated_list1(tag(", "), parse_usize)(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("  Operation: ")(input)?;
        let (input, operation) = Operation::parse(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("  Test: divisible by ")(input)?;
        let (input, test_modulo) = parse_usize(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("    If true: throw to monkey ")(input)?;
        let (input, test_true_target) = parse_usize(input)?;
        let (input, _) = tag("\n")(input)?;

        let (input, _) = tag("    If false: throw to monkey ")(input)?;
        let (input, test_false_target) = parse_usize(input)?;

        Ok((
            input,
            Self {
                worries,
                operation,
                test_modulo,
                test_true_target,
                test_false_target,
            },
        ))
    }

    fn index_to_move_to(&self, new_worry: usize) -> usize {
        if new_worry % self.test_modulo == 0 {
            self.test_true_target
        } else {
            self.test_false_target
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("new = old ")(input)?;
        let (input, operator) = Operator::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, operand) = Operand::parse(input)?;

        Ok((input, Self { operator, operand }))
    }

    fn apply(&self, worry: usize) -> usize {
        let second = match self.operand {
            Operand::Old => worry,
            Operand::Num(num) => num,
        };

        match self.operator {
            Operator::Multiply => worry * second,
            Operator::Add => worry + second,
        }
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Multiply,
    Add,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_multiply = map(tag("*"), |_| Operator::Multiply);
        let parse_add = map(tag("+"), |_| Operator::Add);

        parse_multiply.or(parse_add).parse(input)
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Num(usize),
    Old,
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_old = map(tag("old"), |_: &str| Operand::Old);
        let parse_num = map(parse_usize, Operand::Num);

        parse_old.or(parse_num).parse(input)
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

#[derive(Clone)]
pub struct Day11(Vec<Monkey>);

impl FromInput for Day11 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let lines: Vec<String> = lines.collect();
        Self(
            lines
                .group_by(|_a, b| !b.is_empty())
                .map(|group| {
                    let group = group.join("\n");
                    let (_rest, monkey) = Monkey::parse(group.trim()).expect("Invalid monkey");
                    monkey
                })
                .collect(),
        )
    }
}

impl DaySolution for Day11 {
    fn part_one(&self) -> String {
        let mut monkeys = self.clone();
        let mut inspection_counts = vec![0; self.0.len()];
        let worry_relief = |worry: usize| worry / 3;

        for _round in 0..20 {
            monkeys.run_round(&mut inspection_counts[..], worry_relief);
        }

        calculate_monkey_business(inspection_counts).to_string()
    }

    fn part_two(&self) -> String {
        let mut monkeys = self.clone();
        let mut inspection_counts = vec![0; self.0.len()];
        let full_modulo = self.full_modulo();
        let worry_relief = |worry: usize| worry % full_modulo;

        for _round in 0..10_000 {
            monkeys.run_round(&mut inspection_counts[..], worry_relief);
        }

        calculate_monkey_business(inspection_counts).to_string()
    }
}

impl Day11 {
    fn run_round(
        &mut self,
        inspection_counts: &mut [usize],
        worry_relief: impl Fn(usize) -> usize,
    ) {
        for monkey_index in 0..self.0.len() {
            inspection_counts[monkey_index] += self.0[monkey_index].worries.len();

            let operation = self.0[monkey_index].operation.clone();
            for worry in std::mem::take(&mut self.0[monkey_index].worries) {
                let new_worry = worry_relief(operation.apply(worry));
                let index_to_move_to = self.0[monkey_index].index_to_move_to(new_worry);
                self.0[index_to_move_to].worries.push(new_worry);
            }
        }
    }

    fn full_modulo(&self) -> usize {
        self.0.iter().map(|monkey| monkey.test_modulo).product()
    }
}

fn calculate_monkey_business(mut inspection_counts: Vec<usize>) -> usize {
    inspection_counts.sort();
    inspection_counts.reverse();

    inspection_counts.iter().take(2).product()
}
