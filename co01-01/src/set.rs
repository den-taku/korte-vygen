use std::cmp::PartialEq;
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

impl<T> PartialEq<Set<T>> for Set<T>
where
    T: Eq + std::hash::Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.set == other.set
    }
}

#[cfg(test)]
mod tests_set {
    use crate::set::Set;
    use std::collections::HashSet;

    #[test]
    fn for_set_sub() {
        let set1: HashSet<_> = [4, 5, 6, 7].iter().cloned().collect();
        let set2: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set1 = Set::new(set1);
        let set2 = Set::new(set2);
        assert_eq!(&set1 - &set2, Set::new([6, 7].iter().cloned().collect()));
    }

    #[test]
    fn for_set_min() {
        let set: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set = Set::new(set);
        assert_eq!(set.min(), Some(1));
    }
}
