use std::collections::HashSet;

use crate::{DaySolution, FromInput};

pub struct Day3(Vec<Vec<char>>);

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(lines.map(|line| line.chars().collect()).collect())
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|sack| {
                let (left_half, right_half) = sack.split_at(sack.len() / 2);
                find_common_item(&[left_half, right_half])
            })
            .map(item_priority)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .array_chunks()
            .map(|[elf_1, elf_2, elf_3]| find_common_item(&[elf_1, elf_2, elf_3]))
            .map(item_priority)
            .sum::<usize>()
            .to_string()
    }
}

fn item_priority(item: char) -> usize {
    match item {
        'a'..='z' => (item as usize) - ('a' as usize) + 1,
        'A'..='Z' => (item as usize) - ('A' as usize) + 27,
        _ => panic!("Invalid item provided"),
    }
}

fn find_common_item(groups: &[&[char]]) -> char {
    let mut all_items: HashSet<&char> = HashSet::from_iter(groups[0].iter());

    for group in groups.iter().skip(1) {
        all_items.drain_filter(|i| !group.contains(i));
    }

    **all_items.iter().next().expect("Must have a common item")
}
