fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    let l = getline();
    let n = getline();
    let w = getline();
    let lv: Vec<_> = l.trim().split(' ').collect();
    let nv: Vec<_> = n.trim().split(' ').collect();
    let mut L: i64 = lv[0].parse().unwrap(); // 箱の幅
    let _N: i64 = nv[0].parse().unwrap(); // ブロックの数
    let wl: Vec<_> = w.trim().split(' ').collect(); // 各ブロックの幅

    // ソートする前に文字列を数値に変換する
    let mut W: Vec<_> = wl.iter().map(|f| f.parse::<i64>().unwrap()).collect();

    // 小さい順でブロックを箱に詰めていくのでソートする
    W.sort();

    let mut count: i64 = 0;

    for item in W.iter() {
        let x: i64 = item.to_string().parse().unwrap();
        L = L - x;
        if L < 0 {
            break;
        }
        count += 1;
    }

    println!("{}", count);

}
