#![allow(dead_code)]

use std::collections::HashSet;
use std::ops::{Index, IndexMut, Range, Sub};
use std::convert::From;

fn main(){
    // print n!
    let n = 8;
    path_enumeration_algorithm(n, |set| println!("{:?}", set.to_vec().as_slice()));
}

fn path_enumeration_algorithm<F>(n: usize, f: F)
where
    F: Fn(&SequencialSet<usize>),
{
    if n < 2 {
        panic!("n must be more than 2");
    }
    // 1.
    let mut sequence = SequencialSet::new((1..n + 1).into_iter().collect());
    // first function applied
    f(&sequence);
    let mut i = n - 1;
    // 2
    let mut k;
    while {
        // 2
        k = (&Set::<usize>::from(sequence.extract(i - 1).unwrap() + 1..n + 2)
            - &sequence.extract_with_range(0..i - 1))
            .min()
            .unwrap();
        // 3
        if k <= n {
            sequence[i - 1] = k;
            if i == n {
                f(&sequence);
            }
            if i < n {
                sequence[i] = 0;
                i += 1;
            }
        }
        if k == n + 1 {
            i -= 1;
        }
        i >= 1
    } {}
}

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


#[derive(Debug, Clone)]
pub struct Set<T> {
    set: HashSet<T>,
}

impl<T> Set<T> {
    pub fn new(set: HashSet<T>) -> Self {
        Self { set }
    }
}

impl<T> From<Range<usize>> for Set<T>
where
    T: std::cmp::Eq + std::hash::Hash + From<usize>,
{
    fn from(range: Range<usize>) -> Self {
        let mut set = HashSet::with_capacity(range.end - range.start);
        for e in range {
            set.insert(T::from(e));
        }
        Self::new(set)
    }
}

impl<T> Sub for &Set<T>
where
    T: std::cmp::Eq + std::hash::Hash + Copy,
{
    type Output = Set<T>;

    fn sub(self, other: Self) -> Self::Output {
        let mut set = HashSet::new();
        for x in self.set.difference(&other.set) {
            set.insert(*x);
        }
        Set::new(set)
    }
}

impl<T> Set<T>
where
    T: Ord + Copy,
{
    pub fn min(&self) -> Option<T> {
        if self.set.is_empty() {
            None
        } else {
            let mut ans = *self.set.iter().take(1).next().unwrap();
            for e in self.set.iter() {
                ans = std::cmp::min(ans, *e);
            }
            Some(ans)
        }
    }
}