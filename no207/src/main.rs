use std::io::{self, stdin, Read, StdinLock};

struct Input {
    a: i32,
    b: i32,
}

fn read_input(cin_lock: &mut StdinLock) -> Input {
    Input {
        a: next_token(cin_lock).parse().unwrap(),
        b: next_token(cin_lock).parse().unwrap(),
    }
}

fn solve(input: Input) {
    for c in input.a..=input.b {
        let d = c.to_string().chars().collect::<Vec<char>>();
        if d.contains(&'3') || c % 3 == 0{
            println!("{}", c);
        }
    }
}

fn next_token(cin_lock: &mut io::StdinLock) -> String {
    cin_lock
        .by_ref()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c  | !c.is_whitespace())
        .collect::<String>()
}

fn main() {
    let cin = stdin();
    let mut cin_lock = cin.lock();
    let input = read_input(&mut cin_lock);
    solve(input);
}
