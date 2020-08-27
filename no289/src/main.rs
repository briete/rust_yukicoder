fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
	let gl = getline();
	let lv: Vec<_> = gl.trim().chars().collect();
	let s = lv.iter().filter_map(|v| v.to_string().parse::<i32>().ok());
	println!("{}", s.sum::<i32>());
}
