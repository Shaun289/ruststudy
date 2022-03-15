/// # [기본 수학1](https://www.acmicpc.net/step/8)

/*
use std::io;

fn stdin_readline() -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).unwrap();

    input_str
}
*/

/// ## [손익분기점](https://www.acmicpc.net/problem/1712)
pub fn baekjun_1712(s: String) -> i64 {
    let mut split = s.split_whitespace();
    let vec : Vec<&str> = split.collect();
    if vec.len() < 3 {
        println!("-1");
        return -1;
    }

    let a : i64 = vec[0].parse().unwrap(); // 고정비용
    let b : i64 = vec[1].parse().unwrap(); // 가변비용
    let c : i64 = vec[2].parse().unwrap(); // 한대당 판매가

    // 손익분기점이 존재하지 않음
    if b < 0 || c < 0 || c <= b {
        println!("-1");
        return -1;
    }
    let res = a / (c - b) + 1;
    println!("{}", res);
    res
}


/*
fn main() {
    baekjun_1712(stdin_readline());
}

*/
