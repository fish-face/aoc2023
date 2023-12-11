use std::cmp::max;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use anyhow::{Result};
use thiserror::Error;
use crate::coord::Pt;

#[derive(Debug, Clone, Eq)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

#[derive(Debug, Error)]
pub enum GridErr {
    #[error("Index out of bounds")]
    IndexError
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T>
    where T: Default + Clone
    {
        Grid{width, height, data: vec![T::default(); width * height]}
    }
    pub fn from_data(width: usize, height: usize, data: impl Into<Vec<T>>) -> Grid<T> {
        let mut p = Grid{width, height, data: Vec::new()};
        p.data = data.into();
        p
    }
    // pub fn from_row_data(rows: impl Iterator<Item = impl Into<Vec<T>> /*+ ExactSizeIterator<T>*/>) -> Grid<T> {
    pub fn from_row_data(rows: impl Iterator<Item = impl IntoIterator<Item = T>>) -> Grid<T> {
        let mut grid = Grid{height: 0, width: 0, data: Vec::new()};
        for row in rows {
            for elt in row.into_iter() {
                grid.data.push(elt);
                if grid.height == 0 {
                    grid.width += 1;
                }
            }
            grid.height += 1;
        }
        grid
    }

    pub fn map_from_lines(lines: impl Iterator<Item = String>, f: impl Fn(u8) -> T) -> Grid<T> {
        let mut grid = Grid{height: 0, width: 0, data: Vec::new()};
        for line in lines {
            for elt in line.bytes().map(&f) {
                grid.data.push(elt);
                if grid.height == 0 {
                    grid.width += 1;
                }
            }
            grid.height += 1;
        }
        grid
    }

    pub fn map_from_lines_and_find(
        lines: impl Iterator<Item = String>,
        f: impl Fn(u8) -> T,
        cond: impl Fn(&T) -> bool
    ) -> (Grid<T>, Vec<Pt<usize>>) {
        let mut grid = Grid{height: 0, width: 0, data: Vec::new()};
        let mut found = vec![];
        for (y, line) in lines.enumerate() {
            for (x, elt) in line.bytes().map(&f).enumerate() {
                if cond(&elt) {
                    found.push(Pt(x, y));
                }
                grid.data.push(elt);
                if grid.height == 0 {
                    grid.width += 1;
                }
            }
            grid.height += 1;
        }
        (grid, found)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Pt<usize>, &T)> {
        self.data.iter().enumerate().map(
            |(i, x)| (Pt(i % self.width, i / self.width), x)
        )
    }

    pub fn get(&self, index: Pt<usize>) -> Result<&T, GridErr> {
        let Pt(x, y) = index;
        if x >= self.width || y >= self.height {
            Err(GridErr::IndexError)
        } else {
            Ok(&self[index])
        }
    }

    pub fn contains(&self, p: Pt<usize>) -> bool {
        let Pt(x, y) = p;
        x < self.width && y < self.height
    }

    // WHO THE FUCK NEEDS FUNCTION OVERLOADING, RIGHT???
    pub fn contains_isize(&self, p: Pt<isize>) -> bool {
        let Pt(x, y) = p;
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.as_slice().chunks_exact(self.width)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.as_mut_slice().chunks_exact_mut(self.width)
    }

    pub fn subgrid(&self, top_left: Pt<usize>, bottom_right: Pt<usize>) -> Grid<T>
    where T: Copy
    {
        let Pt(x1, y1) = top_left;
        let Pt(x2, y2) = bottom_right;
        let width = max(0, x2 - x1);
        let height = max(0, y2 - y1);
        let mut data = Vec::with_capacity(width * height);
        for y in y1..y2 {
            for x in x1..x2 {
                data.push(self[Pt(x, y)]);
            }
        }
        Grid::from_data(width, height, data)
    }

    pub fn map<S>(&self, f: impl FnMut(&T) -> S) -> Grid<S>
    {
        Grid::from_data(self.width, self.height, self.data.iter().map(f).collect::<Vec<_>>())
    }

    // pub fn indices_by_row(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = Coord<usize>>> {
    //     (0..self.height)
    //         .map(|y| (0..self.width)
    //             .map(move |x| Coord{x, y}))
    // }

    pub fn columns(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T>> {
        (0..self.width)
            .map(|i| self.data.iter().skip(i).step_by(self.width))
    }

    // pub fn columns_mut(&mut self) -> ColumnsMut<T> {
    //     ColumnsMut::new(self)
    // }

    // pub fn columns_mut(&mut self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &mut T>> {
    //     // zip(0..self.width, repeat(self))
    //     (0..self.width)
    //         .map( |i| self.data.iter_mut().skip(i).step_by_mut(self.width))
    // }

    pub fn try_get(&self, p: Pt<usize>) -> Option<&T> {
        match self.contains(p) {
            true => Some(&self[p]),
            false => None,
        }
    }

    pub fn to_string(&self, sep: Option<&str>) -> String
    where T: ToString
    {
        let sep = sep.unwrap_or(" ");
        self
            .rows()
            .map(|row| row
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(sep)
            ).collect::<Vec<_>>().join("\n")
    }
}

impl<T: Copy> Grid<T> {
    pub fn flood_fill<'a>(&mut self, starts: impl Iterator<Item = &'a Pt<usize>>, cond: impl Fn(&Pt<usize>) -> bool, value: T) {
        let mut to_visit: HashSet<Pt<usize>, RandomState> = HashSet::from_iter(starts.cloned());
        let mut visited = HashSet::new();
        while !to_visit.is_empty() {
            let start = to_visit.iter().next().unwrap().clone();
            to_visit.remove(&start);
            self[start] = value;
            visited.insert(start);
            for neighbour in start.neighbours8() {
                if self.contains(neighbour) && !visited.contains(&neighbour) && cond(&neighbour) {
                    to_visit.insert(neighbour);
                }
            }
        }
    }
}

impl<T> Index<Pt<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pt<usize>) -> &Self::Output {
        let Pt(x, y) = index;
        if x >= self.width || y >= self.height {
            panic!("{} is out of bounds for {}x{} Grid", index, self.width, self.height);
        }
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<Pt<usize>> for Grid<T> {
    fn index_mut(&mut self, index: Pt<usize>) -> &mut Self::Output {
        let Pt(x, y) = index;
        if x >= self.width || y >= self.height {
            panic!("{} is out of bounds for {}x{} Grid", index, self.width, self.height);
        }
        &mut self.data[x + y * self.width]
    }
}

impl<T: PartialEq> PartialEq<Self> for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.data == other.data
    }
}

impl<T: Hash> Hash for Grid<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

// struct Iter<T> {
//     p: usize,
// }
//
// impl Iterator for Iter<T> {
//     type Item = &T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//
//     }
// }

// struct ColumnsMut<'a, T: 'a> {
//     grid: &'a Grid<T>,
//     x: usize,
//     y: usize,
// }
//
// impl<'a, T> ColumnsMut<'a, T> {
//     pub fn new(grid: &Grid<T>) -> ColumnsMut<T> {
//         ColumnsMut{grid, x: 0, y: 0 }
//         // grid.data.iter_mut()
//     }
// }
//
// impl<'a, T> Iterator for ColumnsMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&'a mut self) -> Option<&'a mut T> {
//         let val = self.grid.at(self.x, self.y)?;
//         self.y += 1;
//         if self.y == self.grid.height {
//             self.y = 0;
//             self.x += 1;
//         }
//         Some(val)
//     }
// }