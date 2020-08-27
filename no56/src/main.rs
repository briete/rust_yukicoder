fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    let l = getline();
    let lv: Vec<_> = l.trim().split(' ').collect();
    let d: f64 = lv[0].parse().unwrap();
    let p: f64 = lv[1].parse().unwrap();

    println!("{}", (d + (d * p / 100.0)).floor() as u64);
}
