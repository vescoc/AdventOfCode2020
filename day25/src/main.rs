use std::time::Instant;

use day25::part_1;

fn main() {
    let now = Instant::now();

    println!("part 1: {}", part_1());

    let elapsed = now.elapsed();

    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos(),
    );
}
