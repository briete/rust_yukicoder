fn getline() -> String{
    let mut __ret=String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let gl = getline();
    let glv: Vec<_> = gl.trim().split(' ').collect();
    let n: u32 = glv[0].parse().unwrap(); // n度寝
    let h: u32 = glv[1].parse().unwrap(); // アラームが最初になる時刻の、24時間制の時の値
    let m: u32 = glv[2].parse().unwrap(); // アラーム最初になる時刻の、分の値
    let t: u32 = glv[3].parse().unwrap(); // アラームが繰り返される間隔の分単位の値
    let at_time: u32 = h * 60 + m + t * (n - 1); // 分になおす
    println!("{}", at_time / 60 % 24); // 分を60で割ったら時になる。24は0時なので、24で割ったあまりを求める
    println!("{}", at_time % 60); // 分を60で割ったあまりを求めると、残りの分数になる
}
