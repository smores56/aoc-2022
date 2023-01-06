use std::collections::{HashMap, HashSet};

use crate::util::{BoundingBox, Coordinates, Direction};
use crate::{DaySolution, FromInput};

#[derive(Clone)]
pub struct Day23(HashSet<Coordinates>);

impl FromInput for Day23 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .enumerate()
                .flat_map(|(y, row)| {
                    row.char_indices()
                        .filter_map(|(x, c)| {
                            if c == '#' {
                                Some(Coordinates {
                                    x: x as isize,
                                    y: y as isize,
                                })
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    }
}

impl DaySolution for Day23 {
    fn part_one(&self) -> String {
        let mut elves = self.clone();

        for round in 0..10 {
            elves = elves.move_elves(round);
        }

        let bounds = elves.smallest_containing_rectangle();
        elves.empty_tiles_in_bounds(bounds).to_string()
    }

    fn part_two(&self) -> String {
        let mut elves = self.clone();

        let mut round = 0;
        loop {
            let next_elves = elves.move_elves(round);
            round += 1;

            if elves.0 == next_elves.0 {
                return round.to_string();
            }

            elves = next_elves;
        }
    }
}

impl Day23 {
    fn move_elves(&self, round: usize) -> Self {
        let mut elves = self.clone();

        let mut proposal_counts = HashMap::new();
        for elf in &elves.0 {
            if let Some(proposal) = elves.proposal(*elf, round) {
                proposal_counts.entry(proposal).or_insert((*elf, 0)).1 += 1;
            }
        }

        for (proposal, (elf_coords, proposal_count)) in proposal_counts {
            if proposal_count == 1 {
                elves.0.remove(&elf_coords);
                elves.0.insert(proposal);
            }
        }

        elves
    }

    fn neighbors(&self, coords: Coordinates) -> [bool; 8] {
        let deltas = [
            Coordinates { x: -1, y: -1 },
            Coordinates { x: 0, y: -1 },
            Coordinates { x: 1, y: -1 },
            Coordinates { x: 1, y: 0 },
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 0, y: 1 },
            Coordinates { x: -1, y: 1 },
            Coordinates { x: -1, y: 0 },
        ];

        deltas.map(|delta| {
            let new_coords = coords + delta;
            self.0.contains(&new_coords)
        })
    }

    fn proposal(&self, coords: Coordinates, round: usize) -> Option<Coordinates> {
        let neighbors = self.neighbors(coords);
        if neighbors.iter().all(|n| !n) {
            return None;
        }

        let direction_proposals = [
            ([0, 1, 2], Direction::Up),
            ([4, 5, 6], Direction::Down),
            ([6, 7, 0], Direction::Left),
            ([2, 3, 4], Direction::Right),
        ];

        direction_proposals
            .into_iter()
            .cycle()
            .skip(round % 4)
            .take(4)
            .find_map(|(neighbor_indices, direction)| {
                Some(direction).filter(|_d| {
                    neighbor_indices
                        .into_iter()
                        .all(|neighbor_index| !neighbors[neighbor_index])
                })
            })
            .map(|d| coords + d.normal_vector())
    }

    fn smallest_containing_rectangle(&self) -> BoundingBox {
        BoundingBox::for_coordinates(self.0.iter().cloned())
    }

    fn empty_tiles_in_bounds(&self, bounds: BoundingBox) -> usize {
        let area = (bounds.bottom_right.y - bounds.top_left.y)
            * (bounds.bottom_right.x - bounds.top_left.x);

        area as usize - self.0.len()
    }
}
