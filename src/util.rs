use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl std::ops::Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Coordinates) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<isize> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: isize) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(isize, isize)> for Coordinates {
    fn from(coordinates: (isize, isize)) -> Self {
        Self {
            x: coordinates.0,
            y: coordinates.1,
        }
    }
}

pub struct Matrix<T> {
    pub items: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(size: Coordinates, factory: impl Fn() -> T) -> Self {
        Self {
            items: (0..size.y)
                .map(|_| (0..size.x).map(|_| factory()).collect())
                .collect(),
        }
    }

    pub fn size(&self) -> Coordinates {
        Coordinates {
            x: self.items.get(0).map(|row| row.len()).unwrap_or(0) as isize,
            y: self.items.len() as isize,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, row) in self.items.iter().enumerate() {
            write!(
                f,
                "{}",
                row.iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )?;

            if index < self.items.len() - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl<T> Index<Coordinates> for Matrix<T> {
    type Output = T;

    fn index(&self, coordinates: Coordinates) -> &Self::Output {
        &self.items[coordinates.y as usize][coordinates.x as usize]
    }
}

impl<T> IndexMut<Coordinates> for Matrix<T> {
    fn index_mut(&mut self, coordinates: Coordinates) -> &mut Self::Output {
        &mut self.items[coordinates.y as usize][coordinates.x as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn normal_vector(&self) -> Coordinates {
        match self {
            Direction::Up => Coordinates { x: 0, y: -1 },
            Direction::Down => Coordinates { x: 0, y: 1 },
            Direction::Left => Coordinates { x: -1, y: 0 },
            Direction::Right => Coordinates { x: 1, y: 0 },
        }
    }
}
