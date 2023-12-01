use std::fs;

fn main() {
    println!("Sum of calibration values: {}", fs::read("input.txt").unwrap().split(|b| *b == b'\n').map(|line| {
        let mut digits = line.iter().filter(|&c| *c >= b'0' && *c <= b'9').map(|c| c - b'0');
        let first = digits.next().expect("missing first digit in line");
        let last = digits.last().unwrap_or(first);
        (10 * first as usize) + last as usize
    }).sum::<usize>());
}
