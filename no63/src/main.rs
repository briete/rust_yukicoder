fn getline() -> String{
    let mut __ret=String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let gl = getline();
    let lv: Vec<_> = gl.trim().split(' ').collect();
    let mut l: i64 = lv[0].parse().unwrap();
    let k: i64 = lv[1].parse().unwrap();
    let mut y:i64 = 0;

    loop {
        l-=k*2;
        if l <= 0 {
            break;
        }
        y+=k;
    }

    println!("{}", y);
}
