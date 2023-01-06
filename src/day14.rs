use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use crate::util::{BoundingBox, Coordinates, Matrix};
use crate::{DaySolution, FromInput};

pub struct Day14(Vec<Vec<Coordinates>>);

const POUR_LOCATION: Coordinates = Coordinates { x: 500, y: 0 };

#[derive(PartialEq, Eq, Clone, Copy)]
enum CaveSquare {
    Rock,
    Air,
    Sand,
}

struct Cave {
    grid: Matrix<CaveSquare>,
}

#[derive(PartialEq, Eq)]
enum CaveBottom {
    Floor,
    Void,
}

impl Cave {
    fn from_lines(lines: &Vec<Vec<Coordinates>>, bottom: CaveBottom) -> Self {
        let mut bounds =
            BoundingBox::for_coordinates(lines.iter().flat_map(|line| line.iter().cloned()));
        if bottom == CaveBottom::Floor {
            bounds.bottom_right += Coordinates {
                x: bounds.bottom_right.y,
                y: 2,
            };
        }

        let mut grid = Matrix::new(bounds.bottom_right, || CaveSquare::Air);

        for line in lines {
            for [left, right] in line.array_windows() {
                for coords in left.walk_to(*right).unwrap() {
                    grid[coords] = CaveSquare::Rock;
                }
            }
        }

        if bottom == CaveBottom::Floor {
            let floor = vec![CaveSquare::Rock; grid.size().x as usize];
            let bottom_index = grid.items.len() - 1;
            grid.items[bottom_index] = floor;
        }

        Self { grid }
    }

    fn find_next_grain_location(&self) -> Option<Coordinates> {
        let destinations = [
            Coordinates { x: 0, y: 1 },
            Coordinates { x: -1, y: 1 },
            Coordinates { x: 1, y: 1 },
        ];

        let mut sand = POUR_LOCATION;

        if self.grid[sand] == CaveSquare::Sand {
            return None;
        }

        while let Some(destination) = destinations
            .iter()
            .find(|d| self.grid[sand + **d] == CaveSquare::Air)
        {
            sand += *destination;
            if sand.x >= self.grid.size().x - 1 || sand.y >= self.grid.size().y - 1 {
                return None;
            }
        }

        Some(sand)
    }

    fn grain_count(&self) -> usize {
        self.grid
            .items
            .iter()
            .map(|row| row.iter().filter(|sq| sq == &&CaveSquare::Sand).count())
            .sum::<usize>()
    }
}

fn parse_coordinate_list(input: &str) -> IResult<&str, Vec<Coordinates>> {
    separated_list1(
        tag(" -> "),
        map(
            tuple((parse_isize, tag(","), parse_isize)),
            |(x, _comma, y)| Coordinates { x, y },
        ),
    )(input)
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |d: &str| d.parse())(input)
}

impl FromInput for Day14 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    let (_rest, coordinate_list) =
                        parse_coordinate_list(&line).expect("Invalid coordinate list");
                    coordinate_list
                })
                .collect(),
        )
    }
}

impl DaySolution for Day14 {
    fn part_one(&self) -> String {
        let mut cave = Cave::from_lines(&self.0, CaveBottom::Void);
        while let Some(coords) = cave.find_next_grain_location() {
            cave.grid[coords] = CaveSquare::Sand;
        }

        cave.grain_count().to_string()
    }

    fn part_two(&self) -> String {
        let mut cave = Cave::from_lines(&self.0, CaveBottom::Floor);
        while let Some(coords) = cave.find_next_grain_location() {
            cave.grid[coords] = CaveSquare::Sand;
        }

        cave.grain_count().to_string()
    }
}
