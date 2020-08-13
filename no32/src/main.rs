fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let l = getline();
    let m = getline();
    let n = getline();
    let alv: Vec<_> = l.trim().split(' ').collect();
    let mlv: Vec<_> = m.trim().split(' ').collect();
    let nlv: Vec<_> = n.trim().split(' ').collect();

    // 100 yen
    let mut l: u32 = alv[0].parse().unwrap();

    // 25 yen
    let mut m: u32 = mlv[0].parse().unwrap();

    // 1 yen
    let n: u32 = nlv[0].parse().unwrap();

    m = m + (n / 25);

    l = l + (m / 4);

    let answer = (n % 25) + (m % 4) + (l % 10);

    println!("{}", answer);

}
