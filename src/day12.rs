use crate::util::{Coordinates, Matrix};
use crate::{DaySolution, FromInput};

#[derive(Clone)]
pub struct Day12 {
    grid: Matrix<usize>,
    start: Coordinates,
    end: Coordinates,
}

impl FromInput for Day12 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut start = Coordinates::default();
        let mut end = Coordinates::default();

        let grid = lines
            .into_iter()
            .enumerate()
            .map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_index, c)| match c {
                        'a'..='z' => c as usize - 'a' as usize,
                        'S' => {
                            start = Coordinates {
                                x: col_index as isize,
                                y: row_index as isize,
                            };
                            0
                        }
                        'E' => {
                            end = Coordinates {
                                x: col_index as isize,
                                y: row_index as isize,
                            };
                            25
                        }
                        other => panic!("Unexpected character {other}"),
                    })
                    .collect()
            })
            .collect();

        Self {
            grid: Matrix { items: grid },
            start,
            end,
        }
    }
}

impl DaySolution for Day12 {
    fn part_one(&self) -> String {
        dijkstras(&self.grid, self.start, self.end).to_string()
    }

    fn part_two(&self) -> String {
        (0..self.grid.size().y)
            .flat_map(|y| {
                (0..self.grid.size().x).map(move |x| Coordinates {
                    x: x as isize,
                    y: y as isize,
                })
            })
            .filter(|coords| self.grid[*coords] == 0)
            .map(|coords| dijkstras(&self.grid, coords, self.end))
            .min()
            .expect("No closest coordinates")
            .to_string()
    }
}

fn dijkstras(grid: &Matrix<usize>, start: Coordinates, end: Coordinates) -> usize {
    let mut visited = Matrix::new(grid.size(), || false);
    let mut distances = Matrix::new(grid.size(), || usize::MAX);
    distances[start] = 0;

    let mut queue = vec![start];

    while !queue.is_empty() {
        let (min_index, _coords, distance) = queue
            .iter()
            .enumerate()
            .map(|(index, coords)| (index, coords, distances[*coords]))
            .min_by_key(|(_index, _coords, distance)| *distance)
            .expect("No minimum distance coords");
        let coords = queue.remove(min_index);
        visited[coords] = true;

        let neighbors: Vec<(Coordinates, usize)> = grid
            .neighbors(coords)
            .into_iter()
            .filter(|neighbor| grid[*neighbor] <= grid[coords] + 1 && !visited[*neighbor])
            .map(|neighbor| (neighbor, 1))
            .collect();

        for (neighbor, step) in neighbors {
            let new_distance = distance + step;
            if new_distance < distances[neighbor] {
                distances[neighbor] = new_distance;
                queue.push(neighbor);
            }
        }
    }

    distances[end]
}
