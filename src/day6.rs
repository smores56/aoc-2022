use crate::{DaySolution, FromInput};

pub struct Day6(Vec<char>);

impl FromInput for Day6 {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .next()
                .expect("Must have a single line")
                .chars()
                .collect(),
        )
    }
}

impl DaySolution for Day6 {
    fn part_one(&self) -> String {
        find_start_marker::<4>(&self.0[..]).to_string()
    }

    fn part_two(&self) -> String {
        find_start_marker::<14>(&self.0[..]).to_string()
    }
}

fn find_start_marker<const N: usize>(chars: &[char]) -> usize {
    chars
        .array_windows::<N>()
        .enumerate()
        .find_map(|(index, chars)| chars_are_all_unique(chars).then(|| index + N))
        .expect("Must have packet start marker")
}

fn chars_are_all_unique(chars: &[char]) -> bool {
    !chars
        .iter()
        .enumerate()
        .any(|(index, c)| chars.iter().skip(index + 1).any(|c_2| c == c_2))
}
