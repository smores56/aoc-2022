use crate::{DaySolution, FromInput};

pub struct Day1(Vec<Option<usize>>);

impl FromInput for Day1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(|line| line.parse().ok()).collect())
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        self.group_weights()
            .max()
            .expect("Must have at least one group")
            .to_string()
    }

    fn part_two(&self) -> String {
        let mut weights: Vec<usize> = self.group_weights().collect();
        weights.sort_by_key(|weight| -(*weight as isize));

        weights[0..3].iter().sum::<usize>().to_string()
    }
}

impl Day1 {
    fn group_weights<'d>(&'d self) -> impl 'd + Iterator<Item = usize> {
        self.0
            .group_by(|a, _b| a.is_some())
            .map(|group| group.iter().filter_map(|item| *item).sum::<usize>())
    }
}
