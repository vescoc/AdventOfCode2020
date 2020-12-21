use std::time::Instant;

use day21::part;

fn main() {
    let now = Instant::now();

    let (part_1, part_2) = part();

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);

    let elapsed = now.elapsed();

    println!(
        "elapsed: {}ms ({}ns)",
        elapsed.as_millis(),
        elapsed.as_nanos(),
    );
}
