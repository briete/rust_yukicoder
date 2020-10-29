use std::cmp;

fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let lv: String = getline();
    let lv: Vec<_> = lv.trim().split(' ').collect();
    let v: Vec<_> = lv.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    if n == 1 {
        println!("{}", v[0]);
        return;
    }

    let mut dp: Vec<u32> = vec![0; n];
    dp[0] = v[0];
    dp[1] = cmp::max(v[0], v[1]);

    for i in 2..n {
        dp[i] = cmp::max(dp[i - 1], dp[i - 2] + v[i]);
    }
    println!("{}", dp[n - 1]);
}
