use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/01.txt")?;
    let mut report = String::new();
    file.read_to_string(&mut report)?;
    let nums: Vec<i32> = report
        .lines()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let first = solve_first(&nums);
    println!("first: {}", first);
    let second = solve_second(&nums);
    println!("second: {}", second);
    Ok(())
}

fn solve_first(nums: &[i32]) -> i32 {
    let mut res = 0;
    (1..nums.len()).for_each(|i| {
        if nums[i - 1] < nums[i] {
            res += 1;
        }
    });
    res
}

fn solve_second(nums: &[i32]) -> i32 {
    let mut res = 0;
    (0..nums.len() - 3).for_each(|i| {
        if nums[i] < nums[i + 3] {
            res += 1;
        }
    });
    res
}
