use std::fs;

fn find_first_substring<I: Iterator<Item = char> + Clone>(
    mut iter: I,
    substrings: &[&str],
) -> Option<usize> {
    while let Some(c) = iter.next() {
        for (idx, substr) in substrings.iter().enumerate() {
            let mut substr_chars = substr.chars();
            // Check first character matches...
            if substr_chars.next() == Some(c) {
                // Check if all remaining characters of substring match string
                if iter
                    .clone()
                    .zip(substr_chars)
                    .map(|(a, b)| a == b)
                    .reduce(|a, b| a && b)
                    != Some(false)
                {
                    return Some(idx);
                }
            }
        }
    }

    None
}

fn main() {
    println!(
        "Sum of calibration values: {}",
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| {
                let forward_strings = [
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three",
                    "four", "five", "six", "seven", "eight", "nine",
                ];
                let first_idx = find_first_substring(line.chars(), &forward_strings).unwrap();
                let first_digit = first_idx % 10 + first_idx / 10;

                let rev_strings = [
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "eno", "owt", "eerht",
                    "ruof", "evif", "xis", "neves", "thgie", "enin",
                ];
                let last_idx = find_first_substring(line.chars().rev(), &rev_strings).unwrap();
                let last_digit = last_idx % 10 + last_idx / 10;

                println!("{}: {},{}", line, first_digit, last_digit);

                (first_digit * 10) + last_digit
            })
            .sum::<usize>()
    );
}
