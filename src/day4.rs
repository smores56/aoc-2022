use std::ops::RangeInclusive;

use crate::{DaySolution, FromInput};

pub struct Day4(Vec<AssignmentPair>);

struct AssignmentPair {
    left: RangeInclusive<usize>,
    right: RangeInclusive<usize>,
}

impl AssignmentPair {
    fn from_line(s: String) -> Self {
        let (left, right) = s.split_once(',').expect("Must be pair of ranges");
        let (left_start, left_end) = left
            .split_once('-')
            .expect("Left range must be joined by dash");
        let (right_start, right_end) = right
            .split_once('-')
            .expect("Right range must be joined by dash");

        Self {
            left: (left_start.parse().unwrap()..=left_end.parse().unwrap()),
            right: (right_start.parse().unwrap()..=right_end.parse().unwrap()),
        }
    }
}

impl FromInput for Day4 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(AssignmentPair::from_line).collect())
    }
}

impl DaySolution for Day4 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .filter(|pair| {
                (pair.left.contains(pair.right.start()) && pair.left.contains(pair.right.end()))
                    || (pair.right.contains(pair.left.start())
                        && pair.right.contains(pair.left.end()))
            })
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .filter(|pair| {
                pair.left.contains(pair.right.start())
                    || pair.left.contains(pair.right.end())
                    || pair.right.contains(pair.left.start())
                    || pair.right.contains(pair.left.end())
            })
            .count()
            .to_string()
    }
}
