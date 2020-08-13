fn main() {
    let mut count: u32 = 0;
    let mut max_day: u32 = 0;
    for month in 1..12 {
        match month {
            1|3|5|7|8|10|12 => max_day = 31,
            2 => max_day = 28,
            4|6|9|11 => max_day = 30,
            _ => (),
        };
        for day in 1..max_day {
            let day: Vec<char> = format!("{:02}", day).to_string().chars().collect::<Vec<char>>();
            if month == day[0].to_string().parse::<i32>().unwrap() + day[1].to_string().parse::<i32>().unwrap() {
                count+=1;
            }
        }
    }
    println!("{}", count);
}