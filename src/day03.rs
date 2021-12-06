use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/03.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let input = parse_input(&content);
    let first = solve_first(&input);
    println!("first: {}", first);
    let second = solve_second(&input);
    println!("second: {}", second);
    Ok(())
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|s| s.as_bytes()).collect()
}

fn solve_first(input: &[&[u8]]) -> i32 {
    let len = input.len();
    let cols = input[0].len();
    let mut ones = vec![0; cols];
    for line in input {
        for i in 0..cols {
            if line[i] == b'1' {
                ones[i] += 1;
            }
        }
    }
    let mut gamma = vec![b'0'; cols];
    for i in 0..cols {
        if ones[i] > len / 2 {
            gamma[i] = b'1';
        }
    }
    let gamma = i32::from_str_radix(&String::from_utf8(gamma).unwrap(), 2).unwrap();
    let epsilon = (1 << cols) - 1 - gamma;
    gamma * epsilon
}

fn solve_second(input: &[&[u8]]) -> i32 {
    let width = input[0].len();
    let mut input: Vec<i32> = input
        .into_iter()
        .map(|&num| i32::from_str_radix(&String::from_utf8(num.to_vec()).unwrap(), 2).unwrap())
        .collect();
    input.sort_unstable();
    let oxygen = helper(&input, true, width);
    let co2 = helper(&input, false, width);
    oxygen * co2
}

fn helper(input: &[i32], majority: bool, width: usize) -> i32 {
    // from jimblandy
    let (mut start, mut end) = (0, input.len());
    for bit_pos in (0..width).rev() {
        let mask = 1 << bit_pos;
        let zeros = input[start..end]
            .iter()
            .position(|&num| (num & mask) != 0)
            .unwrap_or(end - start);
        let ones = end - start - zeros;

        if (zeros > ones) == majority {
            end = start + zeros;
        } else {
            start += zeros;
        }
        if end - start == 1 {
            return input[start];
        }
    }
    unreachable!()
}
