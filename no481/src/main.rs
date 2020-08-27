fn getline() -> String{
    let mut __ret=String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let gl = getline();
    let _bv: Vec<_> = gl.trim().split(' ').collect();
    let bv: Vec<i32> = _bv.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 1..=10 {
        if !bv.contains(&i) {
            println!("{}", i);
            break;
        }
    }
}
