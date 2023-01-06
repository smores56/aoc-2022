use std::collections::{HashSet, VecDeque};

use crate::util::{Coordinates, Direction, Matrix};
use crate::{DaySolution, FromInput};

pub struct Day24 {
    valley: Matrix<bool>,
    blizzards: Vec<Blizzard>,
}

#[derive(Debug, Clone)]
struct Blizzard {
    position: Coordinates,
    direction: Direction,
}

impl FromInput for Day24 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut blizzards = vec![];

        let valley = Matrix {
            items: lines
                .enumerate()
                .map(|(y, line)| {
                    line.char_indices()
                        .map(|(x, c)| {
                            let (is_wall, blizzard) = Self::parse_valley_char(x, y, c);
                            if let Some(blizzard) = blizzard {
                                blizzards.push(blizzard);
                            }

                            is_wall
                        })
                        .collect()
                })
                .collect(),
        };

        Self { valley, blizzards }
    }
}

impl DaySolution for Day24 {
    fn part_one(&self) -> String {
        // 11513 is too high
        self.shortest_path().to_string()
    }

    fn part_two(&self) -> String {
        todo!("Solve part two of day 24 using your parsed input")
    }
}

impl Day24 {
    fn parse_valley_char(x: usize, y: usize, c: char) -> (bool, Option<Blizzard>) {
        if c == '#' {
            (true, None)
        } else if c == '.' {
            (false, None)
        } else {
            let direction = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                other => panic!("Unexpected direction: {other}"),
            };

            (
                false,
                Some(Blizzard {
                    position: Coordinates {
                        x: x as isize,
                        y: y as isize,
                    },
                    direction,
                }),
            )
        }
    }

    fn starting_position(&self) -> Coordinates {
        self.valley
            .enumerate()
            .find(|(coords, is_wall)| coords.y == 0 && !*is_wall)
            .expect("Could not find starting position")
            .0
    }

    fn ending_position(&self) -> Coordinates {
        let height = self.valley.size().y - 1;

        self.valley
            .enumerate()
            .find(|(coords, is_wall)| coords.y == height && !*is_wall)
            .expect("Could not find starting position")
            .0
    }

    fn shortest_path(&self) -> isize {
        let start = self.starting_position();
        let end = self.ending_position();

        let mut closest_reached = isize::MAX;
        let mut queue = VecDeque::from([(0, start)]);
        let mut visited = HashSet::new();

        while !queue.is_empty() {
            let (distance, position) = queue.pop_front().unwrap();
            if position == end {
                return distance;
            } else {
                visited.insert((distance, position));
            }

            let distance_from_end = (position - end).manhattan();
            if distance_from_end < closest_reached {
                println!(
                    "Got to {distance_from_end}, queue length {}, visited length {}",
                    queue.len(),
                    visited.len()
                );
                closest_reached = distance_from_end;
            }

            let next_position_deltas = [
                Coordinates { x: 0, y: 0 },
                Coordinates { x: 0, y: -1 },
                Coordinates { x: 0, y: 1 },
                Coordinates { x: -1, y: 0 },
                Coordinates { x: 1, y: 0 },
            ];

            let mut next_positions = next_position_deltas
                .into_iter()
                .map(|delta| (distance + 1, position + delta))
                .filter(|&(next_distance, next_position)| {
                    !visited.contains(&(next_distance, next_position))
                        && !queue.contains(&(next_distance, next_position))
                        && !self.blocked_position(distance, next_position)
                })
                .collect::<Vec<_>>();

            next_positions.sort_by_cached_key(|(_next_distance, next_position)| {
                (*next_position - end).manhattan()
            });

            queue.extend(next_positions);
        }

        isize::MAX
    }

    fn blocked_position(&self, distance: isize, coords: Coordinates) -> bool {
        if !self.valley.in_bounds(coords) || self.valley[coords] {
            return true;
        }

        let inner_size = self.valley.size() - Coordinates { x: 2, y: 2 };

        self.blizzards.iter().any(|blizzard| {
            let current_blizzard_position = match blizzard.direction {
                Direction::Up => Coordinates {
                    x: blizzard.position.x,
                    y: (blizzard.position.y - distance - 1) % inner_size.y + 1,
                },
                Direction::Down => Coordinates {
                    x: blizzard.position.x,
                    y: (blizzard.position.y + distance - 1) % inner_size.y + 1,
                },
                Direction::Left => Coordinates {
                    x: (blizzard.position.x - distance - 1) % inner_size.x + 1,
                    y: blizzard.position.y,
                },
                Direction::Right => Coordinates {
                    x: (blizzard.position.x + distance - 1) % inner_size.x + 1,
                    y: blizzard.position.y,
                },
            };

            coords == current_blizzard_position
        })
    }
}
