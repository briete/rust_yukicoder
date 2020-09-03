fn getline() -> String{
    let mut __ret=String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let x: i32 = getline().trim().to_string().parse().unwrap();
    let y: i32 = getline().trim().to_string().parse().unwrap();
    let l: i32 = getline().trim().to_string().parse().unwrap();

    // 90°回転する回数を求める
    let rotation = if y < 0 {
        2
    } else if x != 0 {
        1
    } else {
        0
    };

    // 移動回数を求める
    let mut count = 0;
    if x.abs() % l == 0 {
        count+=x.abs()/l;
    } else {
        count+=x.abs()/l+1;
    }
    if y.abs() % l == 0 {
        count+=y.abs()/l;
    } else {
        count+=y.abs()/l+1;
    }

    println!("{}", rotation+count);
}
