fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}
fn getline_u32() -> Vec<u32> {
    let gl = getline();
    let v: Vec<_> = gl.trim().split(' ').collect();
    let w: Vec<u32> = v.iter().map(|f| f.parse::<u32>().unwrap()).collect();
    return w;
}

fn main() {
    let _n = getline().trim();
    let w = getline_u32();

    let half_sum: u32 = w.iter().sum() / 2;
}
