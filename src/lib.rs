use std::io;

mod baekjun_basic;
mod baekjun_character;
mod baekjun_math1;
mod cote_array;

// do test :
// cargo test -- --show-output

/// # 백준 테스트를 위한 읽기 함수
/// - 참조 : https://www.acmicpc.net/board/view/85325
#[allow(dead_code)]
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
    fn test_baekjun_11654() {
        assert_eq!(baekjun_character::baekjun_11654(String::from("A")), 65);
        assert_eq!(baekjun_character::baekjun_11654(String::from("C")), 67);
        assert_eq!(baekjun_character::baekjun_11654(String::from("0")), 48);
        assert_eq!(baekjun_character::baekjun_11654(String::from("9")), 57);
        assert_eq!(baekjun_character::baekjun_11654(String::from("a")), 97);
        assert_eq!(baekjun_character::baekjun_11654(String::from("z")), 122);
    }

    #[test]
    fn test_baekjun_10809() {
        let str = String::from("baekjoon");
        let array : [i32; 26] = [1, 0, -1, -1, 2, -1, -1, -1, -1, 4, 3, -1, -1, 7, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
        assert_eq!(baekjun_character::baekjun_10809(str), array);
        let mut array : [i32; 26] = [ -1; 26 ];
        array[0] = 0;
        assert_eq!(baekjun_character::baekjun_10809(String::from("a")), array);
        assert_eq!(baekjun_character::baekjun_10809(String::from("aaa")), array);
        array[1] = 3;
        assert_eq!(baekjun_character::baekjun_10809(String::from("aaabbb")), array);
        array[25] = 6;
        assert_eq!(baekjun_character::baekjun_10809(String::from("aaabbbz")), array);
    }

    #[test]
    fn test_baekjun_1712() {
        assert_eq!(baekjun_math1::baekjun_1712(String::from("1000 70 170")), 11);
        assert_eq!(baekjun_math1::baekjun_1712(String::from("3 2 1")), -1);
        assert_eq!(baekjun_math1::baekjun_1712(String::from("2100000000 9 10")), 2100000001);
    }

    #[test]
    fn test_cote_array() {
        let mut array : [i32] = [1, 3, 5, 7, 9, 11, 13, 15, 17 ,19];
        assert_eq!(cote_array::cote_array_binary(array, 5), 2);
    }
}

