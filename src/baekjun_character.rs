/// # [백준 11654 아스키코드](https://www.acmicpc.net/problem/11654)
/// # 리팩토링
/// - 함수 자체만 가지고 테스트 할수 있도록 하기
/// - 백준에 제출할 수 있도록 read_line도 포함하는 형태로 만들기

mod baekjun;

pub fn baekjun_11654(str: String) -> u8 {
    let result = str.trim().as_bytes()[0];
    println!("{}", result);
    result
}

