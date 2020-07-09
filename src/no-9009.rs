use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;

struct Input {
    a: Vec<usize>,
}

fn read_input(cin_lock: &mut StdinLock) -> Input {
    let n = next_token(cin_lock);
    Input {
        a: (0..n).map(|_| next_token(cin_lock)).collect(),
    }
}

fn solve(input: Input) {
    println!("{}", input.a.iter().fold(0, |acc, v| acc + v))
}

fn next_token<T: FromStr>(cin_lock: &mut StdinLock) -> T {
    cin_lock
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<T>()
        .ok()
        .unwrap()
}

fn main() {
    let cin = stdin();
    let mut cin_lock = cin.lock();
    let input = read_input(&mut cin_lock);
    solve(input);
}
