use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{BinaryHeap, HashMap};

fn part1(reader: io::BufReader<File>) -> io::Result<i32> {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(a_str), Some(b_str)) = (parts.next(), parts.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                left.push(a);
                right.push(b);
            }
        }
    }

    let mut sum = 0;
    while let (Some(value_l), Some(value_r)) = (left.pop(), right.pop()) {
        let diff = value_l - value_r;
        sum += diff.abs();
    }

    Ok(sum)
}

fn part2(reader: io::BufReader<File>) -> io::Result<i32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(a_str), Some(b_str)) = (parts.next(), parts.next()) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                left.push(a);
                *right.entry(b).or_insert(0) += 1;
            }
        }
    }

    let mut sum = 0;
    for value in left.iter(){
        if let Some(count) = right.get(value){
            sum += value*count;
        }
    }

    Ok(sum)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader1 = io::BufReader::new(file);

    let result1 = part1(reader1)?;
    println!("{}", result1);

    let file = File::open("input.txt")?;
    let reader2 = io::BufReader::new(file);

    let result2 = part2(reader2)?;
    println!("{}", result2);

    Ok(())
}
