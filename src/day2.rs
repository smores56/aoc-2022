use crate::{DaySolution, FromInput};

pub struct Day2(Vec<(char, char)>);

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        Self(
            lines
                .map(|line| {
                    (
                        line.chars().nth(0).expect("Must have first choice"),
                        line.chars().nth(2).expect("Must have second choice"),
                    )
                })
                .collect(),
        )
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        self.0
            .iter()
            .map(|(their_choice, my_choice)| score_for_round(*their_choice, *my_choice))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        self.0
            .iter()
            .map(|(their_choice, code)| {
                let my_choice = match (code, their_choice) {
                    ('X', 'A') => 'Z',
                    ('X', 'B') => 'X',
                    ('X', 'C') => 'Y',
                    ('Y', 'A') => 'X',
                    ('Y', 'B') => 'Y',
                    ('Y', 'C') => 'Z',
                    ('Z', 'A') => 'Y',
                    ('Z', 'B') => 'Z',
                    ('Z', 'C') => 'X',
                    _ => panic!("Invalid choice provided"),
                };

                score_for_round(*their_choice, my_choice)
            })
            .sum::<usize>()
            .to_string()
    }
}

fn score_for_round(their_choice: char, my_choice: char) -> usize {
    let shape_score = match my_choice {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid choice made"),
    };

    let battle_score = match (their_choice, my_choice) {
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => panic!("Invalid choice made"),
    };

    shape_score + battle_score
}
