use std::io::{self, stdin, Read, StdinLock};

struct Input {
    n: i32,
    k: i32,
}

fn read_input(cin_lock: &mut StdinLock) -> Input {
    Input {
        n: next_token(cin_lock).parse().unwrap(),
        k: next_token(cin_lock).parse().unwrap(),
    }
}

fn solve(input: Input) {
    if input.n == input.k {
        println!("Drew");
    } else if input.n == 0 && input.k == 1 {
        println!("Won");
    } else if input.n == 0 && input.k == 2 {
        println!("Lost");
    } else if input.n == 1 && input.k == 0 {
        println!("Lost") 
    } else if input.n == 1 && input.k == 2 {
        println!("Won");
    } else if input.n == 2 && input.k == 0 {
        println!("Won")
    } else if input.n == 2 && input.k == 1 {
        println!("Lost")
    }
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

fn main() {
    let cin = stdin();
    let mut cin_lock = cin.lock();
    let input = read_input(&mut cin_lock);
    solve(input);
}
