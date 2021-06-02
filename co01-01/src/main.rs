mod lib;
mod sequence;
mod set;

use crate::sequence::SequencialSet;
use crate::set::Set;
use std::collections::HashSet;

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

fn main() {
    let set1: HashSet<_> = [4, 5, 6, 7].iter().cloned().collect();
    let set2: HashSet<_> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set1 = Set::new(set1);
    let set2 = Set::new(set2);
    println!("{:?}", &set1 - &set2);
    println!("{:?}", set1);
    println!("{:?}", set2);
    println!("{:?}", set2.min());

    let sequence1 = vec![1, 2, 3, 4, 6, 0];
    let sequence2 = vec![2, 2, 3, 4, 6, 5];
    let sequence1 = SequencialSet::new(sequence1);
    let sequence2 = SequencialSet::new(sequence2);
    println!("{:?}", sequence1.extract_with_range(0..4));
    println!("{:?}", sequence2.extract_with_range(4..6));
    println!("{:?}", sequence1.extract(5));
    println!("{:?}", sequence2.extract(5));
    println!("{:?}", sequence2.extract(7));
    println!("{:?}", sequence1.extract_with_range(7..8));

    println!("Path Enumeration Algorithm: ");
    path_enumeration_algorithm(4, |set| println!("{:?}", set.to_vec().as_slice()));
}
