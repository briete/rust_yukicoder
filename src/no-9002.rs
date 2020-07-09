use std::io::{self, Read};

#[derive(Debug)]
struct Input {
    n: i32,
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
        n: next_token(cin_lock).parse().unwrap(),
    }
}

fn fizzbuzz(n: i32) -> Option<&'static str> {
    let fizz = n % 3 == 0;
    let buzz = n % 5 == 0;

    if fizz && buzz {
        Some("FizzBuzz")
    } else if fizz {
        Some("Fizz")
    } else if buzz {
        Some("Buzz")
    } else {
        None
    }
}

fn solve(input: Input, _cin_lock: &mut io::StdinLock) {
    for n in 1..=input.n {
        match fizzbuzz(n) {
            Some(s) => println!("{}", s),
            None => println!("{}", n),
        }
    }
}

fn main() {
    let cin = io::stdin();
    let mut cin_lock = cin.lock();

    let input = read_input(&mut cin_lock);
    solve(input, &mut cin_lock);
}