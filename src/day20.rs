use crate::{DaySolution, FromInput};

#[derive(Clone)]
pub struct Day20(Vec<Node>);

#[derive(Clone)]
struct Node {
    value: isize,
    index: usize,
}

impl FromInput for Day20 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .enumerate()
                .map(|(index, line)| Node {
                    value: line.parse().expect("Invalid number"),
                    index,
                })
                .collect(),
        )
    }
}

impl DaySolution for Day20 {
    fn part_one(&self) -> String {
        let mut data = self.clone();
        data.mix();

        data.sum_grove_coordinates().to_string()
    }

    fn part_two(&self) -> String {
        let decryption_key = 811_589_153;
        let mut data = Self(
            self.0
                .iter()
                .map(|node| Node {
                    index: node.index,
                    value: node.value * decryption_key,
                })
                .collect(),
        );

        for _ in 0..10 {
            data.mix();
        }

        data.sum_grove_coordinates().to_string()
    }
}

impl Day20 {
    fn mix(&mut self) {
        for index in 0..self.0.len() {
            let node_index = self.0.iter().position(|n| n.index == index).unwrap();
            let node = self.0.remove(node_index);

            let len = self.0.len() as isize;
            let new_index = ((node_index as isize + len + node.value) % len + len) % len;
            self.0.insert(new_index as usize, node);
        }
    }

    fn sum_grove_coordinates(&self) -> isize {
        let zero_index = self.0.iter().position(|n| n.value == 0).unwrap();

        [1000, 2000, 3000]
            .into_iter()
            .map(|offset| self.0[(zero_index + offset) % self.0.len()].value)
            .sum::<isize>()
    }
}
