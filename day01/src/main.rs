use std::time::Instant;

use day01::{part_1, part_2};

fn main() {
    let now = Instant::now();

    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());

    let elapsed = now.elapsed();
    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos()
    );
}
