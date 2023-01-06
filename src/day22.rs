use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::IResult;

use crate::util::{BoundingBox, Coordinates, Direction, Matrix};
use crate::{DaySolution, FromInput};

pub struct Day22 {
    map: Matrix<BoardSquare>,
    moves: Vec<BoardMove>,
}

#[derive(Clone, Copy, Debug)]
enum BoardMove {
    Distance(usize),
    TurnLeft,
    TurnRight,
}

impl BoardMove {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(parse_usize, Self::Distance),
            map(tag("L"), |_t| Self::TurnLeft),
            map(tag("R"), |_t| Self::TurnRight),
        ))(input)
    }

    fn parse_all(input: &str) -> IResult<&str, Vec<Self>> {
        many1(Self::parse)(input)
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |d: &str| d.parse())(input)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BoardSquare {
    Solid,
    Open,
    Wall,
}

impl FromInput for Day22 {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        let mut board_rows: Vec<Vec<BoardSquare>> = vec![];
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            board_rows.push(
                line.chars()
                    .map(|c| match c {
                        ' ' => BoardSquare::Solid,
                        '.' => BoardSquare::Open,
                        '#' => BoardSquare::Wall,
                        other => panic!("Invalid square: {other}"),
                    })
                    .collect(),
            );
        }

        let max_width = board_rows.iter().map(|row| row.len()).max().unwrap();
        let map = Matrix {
            items: board_rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .chain(std::iter::repeat(BoardSquare::Solid))
                        .take(max_width)
                        .collect()
                })
                .collect(),
        };

        let (_rest, moves) = BoardMove::parse_all(&lines.next().unwrap()).unwrap();

        Self { map, moves }
    }
}

impl DaySolution for Day22 {
    fn part_one(&self) -> String {
        let move_strategy = |coords: Coordinates, direction: Direction, distance: usize| {
            (
                self.move_to_next_flat_location(coords, direction, distance),
                direction,
            )
        };

        self.walk_over_board(&move_strategy).to_string()
    }

    fn part_two(&self) -> String {
        let move_strategy = |coords: Coordinates, direction: Direction, distance: usize| {
            self.move_to_next_cube_location(coords, direction, distance)
        };

        self.walk_over_board(&move_strategy).to_string()
    }
}

impl Day22 {
    fn walk_over_board(
        &self,
        move_strategy: &impl Fn(Coordinates, Direction, usize) -> (Coordinates, Direction),
    ) -> usize {
        let starting_x = self.map.items[0]
            .iter()
            .position(|sq| sq == &BoardSquare::Open)
            .unwrap();
        let mut coords = Coordinates {
            x: starting_x as isize,
            y: 0,
        };
        let mut direction = Direction::Right;

        // dbg!((coords, direction));

        for move_ in &self.moves {
            match move_ {
                &BoardMove::TurnLeft => {
                    direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Right => Direction::Up,
                    };
                }
                &BoardMove::TurnRight => {
                    direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Down => Direction::Left,
                        Direction::Right => Direction::Down,
                    };
                }
                &BoardMove::Distance(distance) => {
                    (coords, direction) = move_strategy(coords, direction, distance);
                }
            }

            // dbg!((coords, direction));
        }

        score_for_position(coords, direction)
    }

    fn move_to_next_flat_location(
        &self,
        coords: Coordinates,
        direction: Direction,
        distance: usize,
    ) -> Coordinates {
        let mut last_position = coords;

        for _ in 0..distance {
            let mut next_position = last_position + direction.normal_vector();

            loop {
                if self.map.in_bounds(next_position) && self.map[next_position] == BoardSquare::Wall
                {
                    return last_position;
                }

                if !self.map.in_bounds(next_position) {
                    if next_position.y < 0 {
                        next_position.y = self.map.size().y - 1;
                    } else if next_position.y >= self.map.size().y {
                        next_position.y = 0;
                    } else if next_position.x < 0 {
                        next_position.x = self.map.size().x - 1;
                    } else {
                        next_position.x = 0;
                    }
                } else if self.map[next_position] == BoardSquare::Solid {
                    next_position += direction.normal_vector();
                } else {
                    break;
                }
            }

            last_position = next_position;
        }

        last_position
    }

    fn move_to_next_cube_location(
        &self,
        coords: Coordinates,
        direction: Direction,
        distance: usize,
    ) -> (Coordinates, Direction) {
        let mut last_position = coords;

        for _ in 0..distance {
            let mut next_position = last_position + direction.normal_vector();

            loop {
                if self.map.in_bounds(next_position) && self.map[next_position] == BoardSquare::Wall
                {
                    return (last_position, direction);
                }

                if !self.map.in_bounds(next_position)
                    || self.map[next_position] == BoardSquare::Solid
                {
                    // let face = cube_faces()
                    //     .iter()
                    //     .find(|face| face.bounds.contains(last_position))
                    //     .unwrap();
                    // let (next_face_index, next_direction) = face.relations[&direction];
                    // let next_face = cube_faces()[next_face_index];

                    // next_position = 1;

                    if next_position.y < 0 {
                        next_position.y = self.map.size().y - 1;
                    } else if next_position.y >= self.map.size().y {
                        next_position.y = 0;
                    } else if next_position.x < 0 {
                        next_position.x = self.map.size().x - 1;
                    } else {
                        next_position.x = 0;
                    }
                } else {
                    break;
                }
            }

            last_position = next_position;
        }

        (last_position, direction)
    }
}

// fn rotate_position(coords: Coordinates, from: Direction, to: Direction) -> Coordinates {}

fn score_for_position(coords: Coordinates, direction: Direction) -> usize {
    let direction_score = match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    1000 * (coords.y as usize + 1) + 4 * (coords.x as usize + 1) + direction_score
}

struct CubeFace {
    bounds: BoundingBox,
    relations: HashMap<Direction, (usize, Direction)>,
}

fn cube_faces() -> [CubeFace; 6] {
    [
        CubeFace {
            bounds: BoundingBox {
                top_left: Coordinates { x: 50, y: 0 },
                bottom_right: Coordinates { x: 100, y: 50 },
            },
            relations: HashMap::from([
                (Direction::Left, (4, Direction::Left)),
                (Direction::Right, (1, Direction::Left)),
                (Direction::Up, (5, Direction::Left)),
                (Direction::Down, (2, Direction::Up)),
            ]),
        },
        CubeFace {
            bounds: BoundingBox {
                top_left: Coordinates { x: 100, y: 0 },
                bottom_right: Coordinates { x: 150, y: 50 },
            },
            relations: HashMap::from([
                (Direction::Left, (0, Direction::Right)),
                (Direction::Right, (1, Direction::Left)),
                (Direction::Up, (3, Direction::Right)),
                (Direction::Down, (2, Direction::Right)),
            ]),
        },
        CubeFace {
            bounds: BoundingBox {
                top_left: Coordinates { x: 50, y: 50 },
                bottom_right: Coordinates { x: 100, y: 100 },
            },
            relations: HashMap::from([
                (Direction::Left, (4, Direction::Up)),
                (Direction::Right, (1, Direction::Down)),
                (Direction::Up, (0, Direction::Down)),
                (Direction::Down, (3, Direction::Up)),
            ]),
        },
        CubeFace {
            bounds: BoundingBox {
                top_left: Coordinates { x: 50, y: 100 },
                bottom_right: Coordinates { x: 100, y: 150 },
            },
            relations: HashMap::from([
                (Direction::Left, (4, Direction::Right)),
                (Direction::Right, (1, Direction::Right)),
                (Direction::Up, (2, Direction::Down)),
                (Direction::Down, (5, Direction::Right)),
            ]),
        },
        CubeFace {
            bounds: BoundingBox {
                top_left: Coordinates { x: 0, y: 100 },
                bottom_right: Coordinates { x: 50, y: 150 },
            },
            relations: HashMap::from([
                (Direction::Left, (0, Direction::Left)),
                (Direction::Right, (3, Direction::Left)),
                (Direction::Up, (2, Direction::Left)),
                (Direction::Down, (5, Direction::Up)),
            ]),
        },
        CubeFace {
            bounds: BoundingBox {
                top_left: Coordinates { x: 0, y: 150 },
                bottom_right: Coordinates { x: 50, y: 200 },
            },
            relations: HashMap::from([
                (Direction::Left, (0, Direction::Up)),
                (Direction::Right, (3, Direction::Down)),
                (Direction::Up, (4, Direction::Down)),
                (Direction::Down, (1, Direction::Up)),
            ]),
        },
    ]
}
