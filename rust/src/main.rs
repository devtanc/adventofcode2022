mod eight;

use eight::solution::{gold_star_1, gold_star_2};

fn main() {
    let result: String = gold_star_1();
    let result2: String = gold_star_2();
    println!("Result 1 is {}", result);
    println!("Result 2 is {}", result2);
}
