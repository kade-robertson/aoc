#![allow(dead_code)]

use std::{fmt::Display, slice::ChunksExact};

#[derive(Clone)]
pub struct SquareGrid<T> {
    pub size: usize,
    elements: Vec<T>,
}

impl<T> SquareGrid<T> {
    pub fn new(size: usize) -> Self {
        Self { size, elements: Vec::with_capacity(size * size) }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        self.elements.get((row * self.size) + col).unwrap()
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.elements.get_mut((row * self.size) + col).unwrap()
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.elements[(row * self.size) + col] = value;
    }

    pub fn iter_row_col(&self) -> impl Iterator<Item = (&T, usize, usize)> {
        self.elements.iter().enumerate().map(|(i, el)| (el, i / self.size, i % self.size))
    }

    pub fn iter_no_border(&self) -> impl Iterator<Item = (&T, usize, usize)> {
        self.iter_row_col().filter(|(_, row, col)| {
            (1..self.size - 1).contains(row) && (1..self.size - 1).contains(col)
        })
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        self.elements.iter().skip((row * self.size) - 1).take(self.size)
    }

    pub fn iter_rows(&self) -> ChunksExact<T> {
        self.elements.chunks_exact(self.size)
    }
}

impl<T: Default + Clone> SquareGrid<T> {
    pub fn with_default(size: usize) -> Self {
        Self { size, elements: vec![T::default(); size * size] }
    }
}

impl<T> Extend<T> for SquareGrid<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.elements.extend(iter)
    }
}

impl<T: Display> Display for SquareGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, el) in self.elements.iter().enumerate() {
            if i % self.size == 0 {
                write!(f, "[")?;
            }
            write!(f, "{}", el)?;
            if i % self.size == self.size - 1 {
                writeln!(f, "]")?;
            } else {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}
