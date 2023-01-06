use crate::util::{Coordinates, Direction, Matrix};
use crate::{DaySolution, FromInput};

pub struct Day17(Vec<Direction>);

impl FromInput for Day17 {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .next()
                .expect("Must have one line of input")
                .chars()
                .map(|c| {
                    if c == '>' {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                })
                .collect(),
        )
    }
}

impl DaySolution for Day17 {
    fn part_one(&self) -> String {
        let rounds = 2022;
        let height = 5000; // Set close above the tetris looping height

        let mut cave = Cave::new(self.0.clone(), Coordinates { x: 7, y: height });

        cave.drop_n_rocks(rounds, 0).to_string()
    }

    fn part_two(&self) -> String {
        // TODO: this is too low:  1_591_977_077_319
        // TODO: this is too high: 1_591_977_077_354
        let rounds = 1_000_000_000_000;
        let height = 3000; // Set close above the tetris looping height

        let mut cave = Cave::new(self.0.clone(), Coordinates { x: 7, y: height });

        cave.drop_n_rocks(rounds, 0).to_string()
    }
}

struct Cave {
    area: Matrix<bool>,
    gasses: Vec<Direction>,
    gas_index: usize,
}

impl Cave {
    fn new(gasses: Vec<Direction>, size: Coordinates) -> Self {
        Self {
            area: Matrix::new(size, || false),
            gasses,
            gas_index: 0,
        }
    }

    fn get_rock(&self, round: usize) -> Vec<Coordinates> {
        let rocks = [
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, -1), (1, -1), (2, -1), (1, -2)],
            vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2)],
            vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            vec![(0, 0), (1, 0), (0, -1), (1, -1)],
        ];

        let offset = Coordinates {
            x: 2,
            y: self.area.size().y - self.height().0 - 4,
        };

        rocks[round % 5]
            .iter()
            .map(|(x, y)| Coordinates { x: *x, y: *y } + offset)
            .collect::<Vec<_>>()
    }

    fn get_gas_direction(&mut self) -> Direction {
        let gas = self.gasses[self.gas_index % self.gasses.len()];
        self.gas_index += 1;

        gas
    }

    fn height(&self) -> (isize, bool) {
        self.area
            .items
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                if row.iter().any(|sq| *sq) {
                    Some((self.area.size().y - (y as isize), row.iter().all(|sq| *sq)))
                } else {
                    None
                }
            })
            .unwrap_or((0, false))
    }

    fn drop_rock(&mut self, mut rock: Vec<Coordinates>) {
        loop {
            let gas_direction = self.get_gas_direction();
            let _moved_sideways = self.try_move_rock(&mut rock, gas_direction);
            let moved_down = self.try_move_rock(&mut rock, Direction::Down);

            if !moved_down {
                break;
            }
        }

        for coords in rock {
            self.area[coords] = true;
        }
    }

    fn try_move_rock(&self, rock: &mut Vec<Coordinates>, direction: Direction) -> bool {
        let can_move = rock.iter().all(|coords| {
            let new_coords = *coords + direction.normal_vector();
            self.area.in_bounds(new_coords) && !self.area[new_coords]
        });

        if can_move {
            rock.iter_mut()
                .for_each(|coords| *coords += direction.normal_vector());
        }

        can_move
    }

    fn drop_n_rocks(&mut self, rock_count: usize, start_at: usize) -> isize {
        let mut total_height = 0;
        let mut tetris_rounds = vec![];

        for round in start_at..rock_count {
            let (height, is_tetris) = self.height();

            if is_tetris {
                dbg!((round, height, total_height, &tetris_rounds));
                tetris_rounds.push((round, self.gas_index));
                total_height += height;
                self.area = Matrix::new(self.area.size(), || false);

                if let Some((difference, gas_difference)) = Self::find_tetris_period(&tetris_rounds)
                {
                    dbg!(("found it!", difference, gas_difference));
                    let mut index_ = round;
                    while index_ + difference <= rock_count {
                        index_ += difference;
                        total_height += height;
                        self.gas_index += gas_difference;
                    }

                    return total_height + self.drop_n_rocks(rock_count - index_, index_);
                }
            }

            let rock = self.get_rock(round);
            self.drop_rock(rock);
        }

        total_height + self.height().0
    }

    fn find_tetris_period(tetris_rounds: &[(usize, usize)]) -> Option<(usize, usize)> {
        let len = tetris_rounds.len();
        if len < 5 {
            return None;
        }

        dbg!(("here too", tetris_rounds));

        let left_difference = tetris_rounds[len - 2].0 - tetris_rounds[len - 3].0;
        let right_difference = tetris_rounds[len - 1].0 - tetris_rounds[len - 2].0;

        if left_difference == right_difference {
            Some((
                left_difference,
                tetris_rounds[len - 1].1 - tetris_rounds[len - 2].1,
            ))
        } else {
            None
        }
    }
}
