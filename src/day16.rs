use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

use crate::{DaySolution, FromInput};

pub struct Day16(Vec<Valve>);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

impl Valve {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Valve ")(input)?;
        let (input, name) = alpha1(input)?;

        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, flow_rate) = map_res(digit1, |d: &str| d.parse())(input)?;

        let (input, _) = alt((
            tag("; tunnel leads to valve "),
            tag("; tunnels lead to valves "),
        ))(input)?;

        let (input, tunnels) = separated_list1(tag(", "), alpha1)(input)?;

        Ok((
            input,
            Self {
                name: name.to_owned(),
                flow_rate,
                tunnels: tunnels.into_iter().map(ToOwned::to_owned).collect(),
            },
        ))
    }
}

impl FromInput for Day16 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, valve) = Valve::parse(&line).expect("Invalid valve");
                    valve
                })
                .collect(),
        )
    }
}

impl DaySolution for Day16 {
    fn part_one(&self) -> String {
        let start = self
            .0
            .iter()
            .find(|valve| valve.name == "AA")
            .expect("Can't find starting valve");

        let all_distances = self.all_distances();

        let non_empty_valves: Vec<&Valve> =
            self.0.iter().filter(|valve| valve.flow_rate > 0).collect();

        self.find_best_path(start, 30, &non_empty_valves, &all_distances)
            .to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 16 using your parsed input")
    }
}

#[derive(Debug, Clone, Eq, Hash)]
struct PathPoint<'v> {
    valve: &'v Valve,
    time_left: usize,
    total_relief: usize,
    opened_valves: Vec<&'v Valve>,
}

impl<'v> PathPoint<'v> {
    fn neighbors<'p, 'd: 'v>(
        &'p self,
        non_empty_valves: &'v Vec<&Valve>,
        distances: &'d HashMap<(&'d str, &'d str), usize>,
    ) -> impl 'p + Iterator<Item = PathPoint<'v>> {
        non_empty_valves.iter().filter_map(|valve| {
            let distance = distances[&(self.valve.name.as_str(), valve.name.as_str())];
            if distance > self.time_left {
                return None;
            }

            if valve == &self.valve {
                Some(Self {
                    valve,
                    time_left: self.time_left - 1,
                    total_relief: self.total_relief + Self::total_flow(&self.opened_valves),
                    opened_valves: self.opened_valves.clone(),
                })
            } else {
                if self.opened_valves.contains(&valve) {
                    return None;
                }

                let opened_valves: Vec<&Valve> = self
                    .opened_valves
                    .iter()
                    .cloned()
                    .chain(Some(*valve))
                    .collect();

                Some(Self {
                    valve,
                    time_left: self.time_left - distance,
                    total_relief: self.total_relief
                        + distance * Self::total_flow(&self.opened_valves),
                    opened_valves,
                })
            }
        })
    }

    fn total_flow(valves: &Vec<&Valve>) -> usize {
        valves.iter().map(|valve| valve.flow_rate).sum()
    }
}

impl<'v> PartialOrd for PathPoint<'v> {
    fn partial_cmp(&self, other: &PathPoint<'v>) -> Option<Ordering> {
        Some(
            self.total_relief
                .cmp(&other.total_relief)
                .reverse()
                .then(self.time_left.cmp(&other.time_left).reverse()),
        )
    }
}

impl<'v> Ord for PathPoint<'v> {
    fn cmp(&self, other: &PathPoint<'v>) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'v> PartialEq for PathPoint<'v> {
    fn eq(&self, other: &Self) -> bool {
        self.valve == other.valve
            && self.time_left == other.time_left
            && self.total_relief == other.total_relief
            && self.opened_valves.len() == other.opened_valves.len()
            && self
                .opened_valves
                .iter()
                .all(|open| other.opened_valves.contains(open))
    }
}

impl Day16 {
    fn find_best_path(
        &self,
        start: &Valve,
        time: usize,
        non_empty_valves: &Vec<&Valve>,
        distances: &HashMap<(&str, &str), usize>,
    ) -> usize {
        let starting_point = PathPoint {
            valve: start,
            time_left: time,
            total_relief: 0,
            opened_valves: vec![],
        };

        let mut queue = BinaryHeap::from([starting_point.clone()]);
        let mut visited = HashSet::new();
        let mut closest = starting_point;

        while let Some(point) = queue.pop() {
            if point.time_left == 0 {
                return dbg!(point).total_relief;
            } else {
                visited.insert(point.clone());
            }

            if point.time_left < closest.time_left {
                closest = dbg!(point.clone());
                dbg!(queue.len());
            }

            queue.extend(
                point
                    .neighbors(non_empty_valves, distances)
                    .filter(|point| !visited.iter().any(|p| p == point)),
            );
        }

        panic!("Did not find a best path")
    }

    fn all_distances(&self) -> HashMap<(&str, &str), usize> {
        self.0
            .iter()
            .flat_map(|valve1| {
                self.0.iter().map(|valve2| {
                    (
                        (valve1.name.as_str(), valve2.name.as_str()),
                        self.shortest_distance(valve1, valve2),
                    )
                })
            })
            .collect()
    }

    fn shortest_distance(&self, from: &Valve, to: &Valve) -> usize {
        if from == to {
            return 0;
        }

        let mut visited: HashSet<&str> = HashSet::new();
        let mut queue = VecDeque::from_iter([(from, 0)]);

        while let Some((valve, distance)) = queue.pop_front() {
            if valve == to {
                return distance;
            } else {
                visited.insert(&valve.name);
            }

            queue.extend(
                valve
                    .tunnels
                    .iter()
                    .filter(|tunnel| !visited.contains(tunnel.as_str()))
                    .map(|tunnel| (self.get_valve(tunnel), distance + 1)),
            );
        }

        return usize::MAX;
    }

    fn get_valve(&self, name: &str) -> &Valve {
        self.0
            .iter()
            .find(|valve| valve.name == name)
            .expect("Could not find valve")
    }
}
