use std::iter::Extend;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space0};
use nom::combinator::map_res;
use nom::{IResult, Parser};

use crate::{DaySolution, FromInput};

pub struct Day7(Vec<Command>);

#[derive(Clone, Debug)]
struct Command {
    name: String,
    argument: String,
    files: Vec<FileType>,
}

#[derive(Clone, Debug)]
enum FileType {
    File(File),
    Dir(Dir),
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Clone, Debug)]
struct Dir {
    name: String,
    files: Vec<FileType>,
}

impl FileType {
    fn name(&self) -> &str {
        match self {
            FileType::File(file) => &file.name,
            FileType::Dir(dir) => &dir.name,
        }
    }
}

impl Dir {
    fn add_file(&mut self, file: FileType) {
        if !self.files.iter().any(|f| f.name() == file.name()) {
            self.files.push(file);
        } else {
            println!("{:?}, {:?}", file, &self.files);
        }
    }

    fn reconstruct_from_commands(commands: &[Command]) -> Self {
        let mut current_path = vec![];
        let mut root = Dir {
            name: "/".to_owned(),
            files: vec![],
        };

        for command in commands {
            if &command.name == "cd" {
                if &command.argument == ".." {
                    current_path.pop();
                } else if &command.argument == "/" {
                    current_path.clear();
                } else {
                    current_path.push(command.argument.clone());
                }
            } else if &command.name == "ls" {
                let dir = root.find(&current_path);
                for file in &command.files {
                    dir.add_file(file.clone());
                }
            }
        }

        root
    }

    fn find(&mut self, path: &[String]) -> &mut Self {
        match path {
            [] => self,
            [first, rest @ ..] => {
                let child_dir = self
                    .files
                    .iter_mut()
                    .find(|child| child.name() == first)
                    .expect("No child found for path given");

                match child_dir {
                    FileType::Dir(child) => child.find(rest),
                    FileType::File(_file) => panic!("Found file instead of directory"),
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.files
            .iter()
            .map(|file| match file {
                FileType::File(file) => file.size,
                FileType::Dir(dir) => dir.size(),
            })
            .sum()
    }

    fn all_sizes(&self) -> Vec<(&str, usize)> {
        self.files
            .iter()
            .flat_map(|file| match file {
                FileType::Dir(dir) => dir.all_sizes(),
                FileType::File(_file) => vec![],
            })
            .chain(Some((self.name.as_str(), self.size())))
            .collect()
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    let (input, name) = alpha1(input)?;
    let (rest, _) = space0(input)?;

    Ok((
        "",
        Command {
            name: name.to_owned(),
            argument: rest.to_owned(),
            files: vec![],
        },
    ))
}

fn parse_file(input: &str) -> IResult<&str, FileType> {
    fn dir_parser(input: &str) -> IResult<&str, FileType> {
        let (rest, _) = tag("dir ")(input)?;

        Ok((
            "",
            FileType::Dir(Dir {
                name: rest.to_owned(),
                files: vec![],
            }),
        ))
    }

    fn file_parser(input: &str) -> IResult<&str, FileType> {
        let (input, size) = map_res(digit1, |d: &str| d.parse())(input)?;
        let (rest, _) = tag(" ")(input)?;

        Ok((
            "",
            FileType::File(File {
                size,
                name: rest.to_owned(),
            }),
        ))
    }

    dir_parser.or(file_parser).parse(input)
}

impl FromInput for Day7 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut commands = vec![];
        let mut current_command: Option<Command> = None;

        for line in lines {
            if line.starts_with('$') {
                commands.extend(current_command.take());
                let (_rest, current) = parse_command(&line).expect("Invalid command");
                current_command = Some(current);
                continue;
            }

            if let Some(current) = &mut current_command {
                let (_rest, file) = parse_file(&line).expect("Invalid file");
                current.files.push(file);
            }
        }

        commands.extend(current_command.take());

        Self(commands)
    }
}

impl DaySolution for Day7 {
    fn part_one(&self) -> String {
        let directory = Dir::reconstruct_from_commands(&self.0[..]);
        let directory_sizes = directory.all_sizes();

        directory_sizes
            .into_iter()
            .filter(|(name, size)| *size <= 100_000 && name != &"/")
            .map(|(_name, size)| size)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self) -> String {
        let directory = Dir::reconstruct_from_commands(&self.0[..]);
        let mut directory_sizes = directory.all_sizes();
        directory_sizes.sort_by_key(|(_name, size)| *size);

        let total_size = directory_sizes
            .iter()
            .find(|(name, _size)| name == &"/")
            .map(|(_name, size)| size)
            .expect("Couldn't find root directory");

        let filesystem_size = 70_000_000;
        let space_needed = 30_000_000;
        let size_to_delete = space_needed - (filesystem_size - total_size);

        directory_sizes
            .into_iter()
            .map(|(_name, size)| size)
            .find(|size| *size >= size_to_delete)
            .expect("No sufficiently large directory found")
            .to_string()
    }
}
