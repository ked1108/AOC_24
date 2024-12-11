use day4::solution;
use std::time::Instant;
use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let start1 = Instant::now();
    let part1 = solution::part1(&input);
    let duration1 = start1.elapsed();
    println!("{}", part1);
    println!("Took: {:?}", duration1);

    let start2 = Instant::now();
    let part2 = solution::part2(&input);
    let duration2 = start2.elapsed();
    println!("{}", part2);
    println!("Took: {:?}", duration2);

    Ok(())
}
