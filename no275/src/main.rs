fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    // 1行目取得
    let l = getline();
    // 2行目取得
    let ll = getline();
    // 1行目を数値に変換
    let gl: u32 = l.trim().parse().unwrap();
    // 2行目をベクタに変換
    let gll: Vec<_> = ll.trim().split(' ').collect();
    // 2行目のベクタが&strなので、i32数値に変換
    let mut glli: Vec<_> = gll.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    
    // ソート
    glli.sort();

    // 中央値なので、ソート後の配列数から2で割った配列のIndexを取得
    let index = (gl / 2) as usize;
    
    // 偶数の場合は、隣接する2つの数値を足した値を2で割った数が中央値
    if gl % 2 == 0 {
        // f32型にしないと、小数が出力されないため、f32にキャスト
        println!("{}", (glli[index - 1] + glli[index]) as f32 / 2.0)
    } else {
        // 奇数の場合は、真ん中なので、そのまま出力
        println!("{}", glli[index]);
    }

}
