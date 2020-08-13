fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    let l = getline();
    let lv: Vec<_> = l.trim().split(' ').collect();
    let k: i32 = lv[0].parse().unwrap(); // 袋の豆の数
    let n: i32 = lv[1].parse().unwrap(); // 袋の数
    let ll = getline();
    let llv: Vec<_> = ll.trim().split(' ').collect();
    let a: Vec<_> = llv.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mame = k * n;
    let nenrei = a.iter().sum::<i32>();

    if mame < nenrei {
        println!("{}", -1);
    } else {
        println!("{}", mame - nenrei);
    }
}
