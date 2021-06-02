mod algorithm;
mod sequence;
mod set;

use algorithm::path_enumeration_algorithm;

fn main() {
    println!("Path Enumeration Algorithm: ");
    let n = 4;
    path_enumeration_algorithm(n, |set| println!("{:?}", set.to_vec().as_slice()));
}
