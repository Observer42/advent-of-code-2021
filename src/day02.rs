use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn solve() -> Result<()> {
    let mut file = File::open("input/02.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let moves = parse_input(content);
    let first = solve_first(&moves);
    println!("first: {}", first);
    let second = solve_second(&moves);
    println!("second: {}", second);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Moving {
    Forward(i32),
    Down(i32),
}

fn parse_input(input: String) -> Vec<Moving> {
    input
        .lines()
        .map(|s| {
            let mut iter = s.split(' ');
            let num = iter.nth(1).unwrap().parse::<i32>().unwrap();
            if s.starts_with('f') {
                Moving::Forward(num)
            } else if s.starts_with('d') {
                Moving::Down(num)
            } else {
                Moving::Down(-num)
            }
        })
        .collect()
}

fn solve_first(moves: &[Moving]) -> i32 {
    let (x, depth) = moves.into_iter().fold((0, 0), |(x, depth), m| match *m {
        Moving::Forward(n) => (x + n, depth),
        Moving::Down(n) => (x, depth + n),
    });
    x * depth
}

fn solve_second(moves: &[Moving]) -> i32 {
    let (x, depth, _aim) = moves
        .into_iter()
        .fold((0, 0, 0), |(x, depth, aim), m| match *m {
            Moving::Forward(n) => (x + n, depth + aim * n, aim),
            Moving::Down(n) => (x, depth, aim + n),
        });
    x * depth
}
