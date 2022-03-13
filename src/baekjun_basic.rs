// 참고자료 : https://www.acmicpc.net/board/view/85325
use std::io;

// https://www.acmicpc.net/problem/1330
pub fn baekjun_1330() -> bool {
    let mut in_str = String::new();

    io::stdin().read_line(&mut in_str).unwrap();

    let mut split = in_str.split_ascii_whitespace();

    let n1 = split.next().unwrap().parse::<usize>().unwrap();
    let n2 = split.next().unwrap().parse::<usize>().unwrap();

    if n1 < n2 {
        println!("<");
    }
    else if n1 > n2 {
        println!(">");
    }
    else  {
        println!("==");
    }
    true
}

// https://www.acmicpc.net/problem/2557
pub fn baekjun_2557() -> bool {
    let hello_world = String::from("hello world!");
    println!("{}", hello_world);
    true
}

