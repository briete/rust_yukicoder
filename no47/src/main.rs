fn getline() -> String{
    let mut __ret=String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let gl = getline().trim().to_string();
    let n: i32 = gl.parse().unwrap();
    let mut cookie: i32 = 1;
    let mut count: i32 = 0;
    loop {
        if cookie >= n {
            break;
        }
        cookie*=2;
        count+=1;
    }
    println!("{}", count);
}
