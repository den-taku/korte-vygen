#![allow(dead_code)]

use crate::set::Set;
use std::collections::HashSet;
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Clone)]
pub struct SequencialSet<T> {
    sequence: Vec<T>,
}

impl<T> Index<usize> for SequencialSet<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.sequence[index]
    }
}

impl<T> IndexMut<usize> for SequencialSet<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.sequence[index]
    }
}

impl<T> SequencialSet<T>
where
    T: Clone + std::cmp::Eq + std::hash::Hash,
{
    pub fn to_set(&self) -> Set<T> {
        Set::new(self.sequence.iter().cloned().collect())
    }
}

impl<T> SequencialSet<T> 
where
    T: Clone
{
    pub fn to_vec(&self) -> Vec<T> {
        self.sequence.clone()
    }
}

impl<T> SequencialSet<T>
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    pub fn extract_with_range(&self, range: Range<usize>) -> Set<T> {
        let mut set = HashSet::with_capacity(self.sequence.len());
        for e in self
            .sequence
            .iter()
            .take(range.end - range.start)
            .skip(range.start)
        {
            set.insert(*e);
        }
        Set::<T>::new(set)
    }
}

impl<T> SequencialSet<T>
where
    T: Copy,
{
    pub fn extract(&self, index: usize) -> Option<T> {
        if index < self.sequence.len() {
            Some(self.sequence[index])
        } else {
            None
        }
    }
}

impl<T> SequencialSet<T> {
    pub fn new(sequence: Vec<T>) -> Self {
        Self { sequence }
    }
}
