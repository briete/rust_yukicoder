use std::io::{self, Read};

#[derive(Debug)]
struct Input {
    s: i32,
    f: i32,
}

fn next_token(cin_lock: &mut io::StdinLock) -> String {
    cin_lock
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>()
}

fn read_input(cin_lock: &mut io::StdinLock) -> Input {
    Input {
        s: next_token(cin_lock).parse().unwrap(),
        f: next_token(cin_lock).parse().unwrap(),
    }
}

fn solve(input: Input, _cin_lock: &mut io::StdinLock) {
    let answer = (input.s / input.f) + 1;
    println!("{}", answer);
}

fn main() {
    let cin = io::stdin();
    let mut cin_lock = cin.lock();

    let input = read_input(&mut cin_lock);
    solve(input, &mut cin_lock);
}
