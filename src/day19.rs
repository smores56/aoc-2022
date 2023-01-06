use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use crate::{DaySolution, FromInput};

pub struct Day19(Vec<Blueprint>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("ore"), |_r| Resource::Ore),
            map(tag("clay"), |_r| Resource::Clay),
            map(tag("obsidian"), |_r| Resource::Obsidian),
            map(tag("geode"), |_r| Resource::Geode),
        ))(input)
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    robots: HashMap<Resource, RobotBlueprint>,
}

impl Blueprint {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Blueprint ")(input)?;
        let (input, id) = parse_usize(input)?;
        let (input, _) = tag(": ")(input)?;

        let (input, robots) = separated_list1(tag(". "), RobotBlueprint::parse)(input)?;

        Ok((
            input,
            Self {
                id,
                robots: robots.into_iter().collect(),
            },
        ))
    }

    fn quality_level(&self, total_minutes: usize) -> usize {
        dbg!(self.id) * dbg!(self.max_geodes_after_minutes(total_minutes))
    }

    fn max_geodes_after_minutes(&self, total_minutes: usize) -> usize {
        let robots = HashMap::from([
            (Resource::Ore, 1),
            (Resource::Clay, 0),
            (Resource::Obsidian, 0),
            (Resource::Geode, 0),
        ]);
        let resources = HashMap::from([
            (Resource::Ore, 0),
            (Resource::Clay, 0),
            (Resource::Obsidian, 0),
            (Resource::Geode, 0),
        ]);

        self.max_geodes_after_minutes_inner(robots, resources, total_minutes)
    }

    fn max_geodes_after_minutes_inner(
        &self,
        robots: HashMap<Resource, usize>,
        resources: HashMap<Resource, usize>,
        total_minutes: usize,
    ) -> usize {
        if total_minutes == 0 {
            return *resources.get(&Resource::Geode).unwrap_or(&0);
        }

        self.robot_options(&robots, resources)
            .map(|(new_robots, new_resources)| {
                self.max_geodes_after_minutes_inner(new_robots, new_resources, total_minutes - 1)
            })
            .max()
            .unwrap_or(0)
    }

    fn robot_options<'a>(
        &'a self,
        robots: &'a HashMap<Resource, usize>,
        resources: HashMap<Resource, usize>,
    ) -> impl 'a + Iterator<Item = (HashMap<Resource, usize>, HashMap<Resource, usize>)> {
        self.affordable_amounts(resources, Resource::Geode)
            .flat_map(move |(geodes, resources2)| {
                self.affordable_amounts(resources2, Resource::Obsidian)
                    .flat_map(move |(obsidians, resources3)| {
                        self.affordable_amounts(resources3, Resource::Clay)
                            .flat_map(move |(clays, resources4)| {
                                self.affordable_amounts(resources4, Resource::Ore).map(
                                    move |(ores, resources5)| {
                                        let remaining_resources = resources5
                                            .into_iter()
                                            .map(|(resource, amount)| {
                                                (resource, amount + robots[&resource])
                                            })
                                            .collect();

                                        let new_robots = HashMap::from([
                                            (
                                                Resource::Geode,
                                                *robots.get(&Resource::Geode).unwrap_or(&0)
                                                    + geodes,
                                            ),
                                            (
                                                Resource::Obsidian,
                                                *robots.get(&Resource::Obsidian).unwrap_or(&0)
                                                    + obsidians,
                                            ),
                                            (
                                                Resource::Clay,
                                                *robots.get(&Resource::Clay).unwrap_or(&0) + clays,
                                            ),
                                            (
                                                Resource::Ore,
                                                *robots.get(&Resource::Ore).unwrap_or(&0) + ores,
                                            ),
                                        ]);

                                        (new_robots, remaining_resources)
                                    },
                                )
                            })
                    })
            })
    }

    fn affordable_amounts<'a>(
        &'a self,
        resources: HashMap<Resource, usize>,
        resource: Resource,
    ) -> impl 'a + Iterator<Item = (usize, HashMap<Resource, usize>)> {
        let amount = self.robots[&resource].amount_affordable(&resources);

        Some(0)
            .into_iter()
            .chain(Some(amount).filter(|a| a != &0))
            .filter(move |a| {
                if (resource == Resource::Geode || resource == Resource::Obsidian) && amount > 0 {
                    a != &0
                } else {
                    true
                }
            })
            .map(move |taken| {
                let remaining_resources =
                    self.robots[&resource].take_resources(resources.clone(), taken);
                (taken, remaining_resources)
            })
    }
}

#[derive(Debug)]
struct RobotBlueprint {
    ingredients: HashMap<Resource, usize>,
}

impl RobotBlueprint {
    fn parse(input: &str) -> IResult<&str, (Resource, Self)> {
        let (input, _) = tag("Each ")(input)?;
        let (input, resource) = Resource::parse(input)?;
        let (input, _) = tag(" robot costs ")(input)?;

        let (input, ingredients) = separated_list1(
            tag(" and "),
            tuple((parse_usize, tag(" "), Resource::parse)),
        )(input)?;

        Ok((
            input,
            (
                resource,
                Self {
                    ingredients: ingredients
                        .into_iter()
                        .map(|(amount, _, resource)| (resource, amount))
                        .collect(),
                },
            ),
        ))
    }

    fn amount_affordable(&self, resources: &HashMap<Resource, usize>) -> usize {
        self.ingredients
            .iter()
            .map(|(ingredient, required)| {
                resources.get(ingredient).unwrap_or(&0).div_floor(*required)
            })
            .min()
            .unwrap_or(0)
    }

    fn take_resources(
        &self,
        resources: HashMap<Resource, usize>,
        times: usize,
    ) -> HashMap<Resource, usize> {
        resources
            .into_iter()
            .map(|(resource, remaining)| {
                (
                    resource,
                    remaining - times * self.ingredients.get(&resource).unwrap_or(&0),
                )
            })
            .collect()
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse())(input)
}

impl FromInput for Day19 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, blueprint) = Blueprint::parse(&line).expect("Invalid blueprint");
                    blueprint
                })
                .collect(),
        )
    }
}

impl DaySolution for Day19 {
    fn part_one(&self) -> String {
        // 13104 is too high

        let total_minutes = 15;

        // Blueprint 1:
        //   Each ore robot costs 4 ore.
        //   Each clay robot costs 2 ore.
        //   Each obsidian robot costs 3 ore and 14 clay.
        //   Each geode robot costs 2 ore and 7 obsidian.

        after N minutes = best option after N - 1 minutes

        self.0
            .iter()
            .map(|blueprint| blueprint.quality_level(total_minutes))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 19 using your parsed input")
    }
}
