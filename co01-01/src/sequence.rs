#![allow(dead_code)]

use crate::set::Set;
use std::cmp::PartialEq;
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
    T: Clone,
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
            .skip(range.start)
            .take(range.end - range.start)
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

impl<T> PartialEq<SequencialSet<T>> for SequencialSet<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

#[cfg(test)]
mod tests_sequence {
    use crate::sequence::SequencialSet;
    use crate::set::Set;
    use std::collections::HashSet;

    #[test]
    fn for_sequence_new() {
        let sequence1 = vec![1, 2, 3, 4, 6, 0];
        let sequence2 = vec![2, 2, 3, 4, 6, 5];
        let sequence1 = SequencialSet::new(sequence1);
        let sequence2 = SequencialSet::new(sequence2);

        assert_eq!(
            sequence1.extract_with_range(0..4),
            Set::new([1, 2, 3, 4].iter().cloned().collect())
        );
        assert_eq!(
            sequence2.extract_with_range(4..6),
            Set::new([6, 5].iter().cloned().collect())
        );
        assert_eq!(sequence1.extract(5), Some(0));
        assert_eq!(sequence2.extract(5), Some(5));
        assert_eq!(sequence2.extract(7), None);
        assert_eq!(
            sequence1.extract_with_range(7..8),
            Set::new(HashSet::<usize>::new())
        );
    }
}
