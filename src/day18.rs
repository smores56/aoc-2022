use std::collections::HashSet;
use std::ops::Add;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

use crate::{DaySolution, FromInput};

#[derive(Clone)]
pub struct Day18(HashSet<Cube>);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, x) = parse_isize(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = parse_isize(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, z) = parse_isize(input)?;

        Ok((input, Self { x, y, z }))
    }

    fn all_directions() -> [Cube; 6] {
        [
            Cube::from((0, 0, 1)),
            Cube::from((0, 0, -1)),
            Cube::from((0, 1, 0)),
            Cube::from((0, -1, 0)),
            Cube::from((1, 0, 0)),
            Cube::from((-1, 0, 0)),
        ]
    }

    fn all_neighbors(&self) -> [Cube; 6] {
        Self::all_directions().map(|delta| *self + delta)
    }
}

impl From<(isize, isize, isize)> for Cube {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl Add for Cube {
    type Output = Self;

    fn add(self, rhs: Cube) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |d: &str| d.parse())(input)
}

impl FromInput for Day18 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, cube) = Cube::parse(&line).expect("Invalid cube");
                    cube
                })
                .collect(),
        )
    }
}

impl DaySolution for Day18 {
    fn part_one(&self) -> String {
        self.surface_area().to_string()
    }

    fn part_two(&self) -> String {
        let mut lava = self.clone();
        lava.fill_in_gaps();

        lava.surface_area().to_string()
    }
}

#[derive(Debug)]
struct CubeBounds {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Day18 {
    fn surface_area(&self) -> usize {
        self.0
            .iter()
            .map(|cube| {
                cube.all_neighbors()
                    .into_iter()
                    .filter(|neighbor| !self.0.contains(neighbor))
                    .count()
            })
            .sum()
    }

    fn bounds(&self) -> CubeBounds {
        self.0.iter().fold(
            CubeBounds {
                min_x: isize::MAX,
                max_x: isize::MIN,
                min_y: isize::MAX,
                max_y: isize::MIN,
                min_z: isize::MAX,
                max_z: isize::MIN,
            },
            |bounds, cube| CubeBounds {
                min_x: std::cmp::min(bounds.min_x, cube.x),
                max_x: std::cmp::max(bounds.max_x, cube.x),
                min_y: std::cmp::min(bounds.min_y, cube.y),
                max_y: std::cmp::max(bounds.max_y, cube.y),
                min_z: std::cmp::min(bounds.min_z, cube.z),
                max_z: std::cmp::max(bounds.max_z, cube.z),
            },
        )
    }

    fn fill_in_gaps(&mut self) {
        let bounds = self.bounds();

        for x in bounds.min_x..=bounds.max_x {
            for y in bounds.min_y..=bounds.max_y {
                for z in bounds.min_z..=bounds.max_z {
                    let cube = Cube { x, y, z };
                    if self.0.contains(&cube) {
                        continue;
                    }

                    if !self.is_a_way_out(&bounds, &cube) {
                        self.0.insert(cube);
                    }
                }
            }
        }
    }

    fn is_a_way_out(&self, bounds: &CubeBounds, cube: &Cube) -> bool {
        let mut queue = vec![*cube];
        let mut visited: HashSet<Cube> = HashSet::new();

        while let Some(c) = queue.pop() {
            if !bounds.contains(&c) {
                return true;
            } else {
                visited.insert(c);
            }

            queue.extend(
                Cube::all_directions()
                    .into_iter()
                    .map(|direction| c + direction)
                    .filter(|neighbor| !self.0.contains(neighbor) && !visited.contains(neighbor)),
            );
        }

        false
    }
}

impl CubeBounds {
    fn contains(&self, cube: &Cube) -> bool {
        self.min_x <= cube.x
            && cube.x <= self.max_x
            && self.min_y <= cube.y
            && cube.y <= self.max_y
            && self.min_z <= cube.z
            && cube.z <= self.max_z
    }
}
