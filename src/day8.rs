use crate::util::{Coordinates, Direction, Matrix};
use crate::{DaySolution, FromInput};

pub struct Day8(Matrix<isize>);

impl FromInput for Day8 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(Matrix {
            items: lines
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).expect("Invalid height") as isize)
                        .collect()
                })
                .collect(),
        })
    }
}

impl DaySolution for Day8 {
    fn part_one(&self) -> String {
        let mut visible = Matrix::new(self.0.size(), || false);

        for direction in Direction::ALL {
            self.walk_visible_path(&mut visible, direction);
        }

        visible
            .items
            .iter()
            .map(|row| row.iter().filter(|vis| **vis).count())
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let all_coordinates = (1..(self.0.size().x - 1))
            .flat_map(|x| (1..(self.0.size().y - 1)).map(move |y| (x, y)))
            .map(Coordinates::from);

        all_coordinates
            .map(|coords| self.calculate_scenic_score(coords))
            .max()
            .expect("No maximum found")
            .to_string()
    }
}

impl Day8 {
    fn walk_visible_path(&self, visible: &mut Matrix<bool>, direction: Direction) {
        let region_size = visible.size();
        for start in starting_coordinates_for_direction(region_size, direction) {
            let mut max_height = -1;
            let mut coords = start;
            visible[coords] = true;

            let length = match direction {
                Direction::Left | Direction::Right => region_size.x,
                Direction::Up | Direction::Down => region_size.y,
            };

            for _ in 0..length {
                let height = self.0[coords];
                visible[coords] = height > max_height || visible[coords];
                max_height = std::cmp::max(max_height, height);
                coords += direction.normal_vector();
            }
        }
    }

    fn calculate_scenic_score(&self, coordinates: Coordinates) -> isize {
        let mut score = 1;

        for direction in Direction::ALL {
            let max_distance = match direction {
                Direction::Up => coordinates.y,
                Direction::Down => self.0.size().y - coordinates.y - 1,
                Direction::Left => coordinates.x,
                Direction::Right => self.0.size().x - coordinates.x - 1,
            };

            let height = self.0[coordinates];
            let mut travelled = 0;
            for _ in 0..max_distance {
                travelled += 1;
                let coords_to_check = coordinates + direction.normal_vector() * travelled;
                if self.0[coords_to_check] >= height {
                    break;
                }
            }

            score *= travelled;
        }

        score
    }
}

fn starting_coordinates_for_direction(
    region_size: Coordinates,
    direction: Direction,
) -> Box<dyn Iterator<Item = Coordinates>> {
    match direction {
        Direction::Up => Box::new(
            (0..region_size.x)
                .zip(std::iter::repeat(region_size.y - 1))
                .map(Coordinates::from),
        ),
        Direction::Down => Box::new(
            (0..region_size.x)
                .zip(std::iter::repeat(0))
                .map(Coordinates::from),
        ),
        Direction::Left => Box::new(
            std::iter::repeat(region_size.x - 1)
                .zip(0..region_size.y)
                .map(Coordinates::from),
        ),
        Direction::Right => Box::new(
            std::iter::repeat(0)
                .zip(0..region_size.y)
                .map(Coordinates::from),
        ),
    }
}
