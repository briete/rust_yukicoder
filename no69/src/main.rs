fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    let gla = getline();
    let glb = getline();
    let mut a: Vec<_> = gla.trim().chars().collect();
    let mut b: Vec<_> = glb.trim().chars().collect();

    a.sort();
    b.sort();

    if a == b {
        println!("{}", "YES");
    } else {
        println!("{}", "NO");
    }
}
