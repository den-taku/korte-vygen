mod lib;
mod sequence;
mod set;

use crate::sequence::SequencialSet;
use crate::set::Set;

fn main() {
    println!("Path Enumeration Algorithm: ");
    let n = 4;
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