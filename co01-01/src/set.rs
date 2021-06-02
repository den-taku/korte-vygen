use std::collections::HashSet;
use std::convert::From;
use std::ops::Range;
use std::ops::Sub;

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
