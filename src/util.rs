use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl Coordinates {
    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    pub fn direction(&self) -> Option<Direction> {
        match (self.x, self.y) {
            (0, y) if y < 0 => Some(Direction::Up),
            (0, y) if y > 0 => Some(Direction::Down),
            (x, 0) if x < 0 => Some(Direction::Left),
            (x, 0) if x > 0 => Some(Direction::Right),
            _other => None,
        }
    }

    pub fn walk_to(&self, to: Coordinates) -> Option<impl Iterator<Item = Coordinates>> {
        let direction = (to - *self).direction()?.normal_vector();

        Some(std::iter::successors(Some(*self), move |coords| {
            if coords == &to {
                None
            } else {
                Some(*coords + direction)
            }
        }))
    }
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

#[derive(Debug, Clone)]
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

    pub fn in_bounds(&self, coords: Coordinates) -> bool {
        BoundingBox {
            top_left: Coordinates { x: 0, y: 0 },
            bottom_right: self.size(),
        }
        .contains(coords)
    }

    pub fn neighbors(&self, coords: Coordinates) -> Vec<Coordinates> {
        Direction::ALL
            .into_iter()
            .filter_map(|direction| {
                Some(coords + direction.normal_vector())
                    .filter(|new_coords| self.in_bounds(*new_coords))
            })
            .collect()
    }

    pub fn enumerate<'m>(&'m self) -> impl Iterator<Item = (Coordinates, &'m T)> {
        self.items.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, item)| {
                (
                    Coordinates {
                        x: x as isize,
                        y: y as isize,
                    },
                    item,
                )
            })
        })
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

#[derive(Debug)]
pub struct BoundingBox {
    pub top_left: Coordinates,
    pub bottom_right: Coordinates,
}

impl BoundingBox {
    pub fn for_coordinates<'c>(all_coords: impl Iterator<Item = Coordinates>) -> Self {
        let mut top_left = Coordinates {
            x: isize::MAX,
            y: isize::MAX,
        };
        let mut bottom_right = Coordinates {
            x: isize::MIN,
            y: isize::MIN,
        };

        for coords in all_coords {
            top_left.x = std::cmp::min(coords.x, top_left.x);
            top_left.y = std::cmp::min(coords.y, top_left.y);

            bottom_right.x = std::cmp::max(coords.x, bottom_right.x);
            bottom_right.y = std::cmp::max(coords.y, bottom_right.y);
        }

        Self {
            top_left,
            bottom_right: bottom_right + Coordinates { x: 1, y: 1 },
        }
    }

    pub fn contains(&self, coords: Coordinates) -> bool {
        coords.x >= self.top_left.x
            && coords.x < self.bottom_right.x
            && coords.y >= self.top_left.y
            && coords.y < self.bottom_right.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
