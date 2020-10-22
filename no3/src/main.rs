use std::collections::VecDeque;

fn getline() -> String{
	let mut __ret=String::new();
	std::io::stdin().read_line(&mut __ret).ok();
	return __ret;
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut masu: Vec<i32> = vec![-1; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    masu[1] = 1;
    queue.push_back(1);

    while !queue.is_empty() {
        let current_position = queue.pop_front().unwrap();
        let bit_count_ones: usize = current_position.count_ones() as usize;
        if current_position - bit_count_ones > 0 && masu[current_position - bit_count_ones] == -1  {
            masu[current_position - bit_count_ones] = masu[current_position] + 1;
            queue.push_back(current_position - bit_count_ones);
        }
        if current_position + bit_count_ones <= n && masu[current_position + bit_count_ones] == -1 {
            masu[current_position + bit_count_ones] = masu[current_position] + 1;
            queue.push_back(current_position + bit_count_ones);
        }
    }

    //　結局最後のマスに到達しているかしていないかなのでそれを出力する（到達していない場合は -1）
    println!("{}", masu[n]);
}
