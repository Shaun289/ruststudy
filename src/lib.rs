use std::io;

mod baekjun_basic;
mod baekjun_character;

// do test :
// cargo test -- --show-output

/// # 백준 테스트를 위한 읽기 함수
/// - 참조 : https://www.acmicpc.net/board/view/85325
fn stdin_readline() -> String {
    let mut input_str = String::new();

    io::stdin().read_line(&mut input_str).unwrap();

    input_str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baekjun_basic() {
        assert!(baekjun_basic::baekjun_2557());
        assert!(baekjun_basic::baekjun_1330());
    }

    #[test]
    fn test_baekjun_character() {
        assert_eq!(baekjun_character::baekjun_11654(String::from("A")), 65);
        assert_eq!(baekjun_character::baekjun_11654(String::from("C")), 67);
        assert_eq!(baekjun_character::baekjun_11654(String::from("0")), 48);
        assert_eq!(baekjun_character::baekjun_11654(String::from("9")), 57);
        assert_eq!(baekjun_character::baekjun_11654(String::from("a")), 97);
        assert_eq!(baekjun_character::baekjun_11654(String::from("z")), 122);
    }
}

