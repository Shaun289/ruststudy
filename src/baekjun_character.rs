
/// # [백준 11654 아스키코드](https://www.acmicpc.net/problem/11654)
pub fn baekjun_11654(str: String) -> u8 {
    let result = str.trim().as_bytes()[0];
    println!("{}", result);
    result
}

/// # [백준 10809 알파벳찾기](https://www.acmicpc.net/problem/10809)
pub fn baekjun_10809(str: String) -> [i32; 26] {
    let mut array : [i32; 26] = [-1; 26];
    let a_val : i32 = 'a' as i32;

    for (i, c) in str.trim().chars().enumerate() {
        let c_val = c as i32;
        if c_val < a_val  || c_val >= a_val+26 {
            continue;
        }
        let index : usize = (c_val - a_val) as usize;
        if array[index] == -1 {
            array[index] = i as i32;
        }
    }


    let str_nums: Vec<String> = array.iter()
        .map(|n| n.to_string())
        .collect();
    println!("{}", str_nums.join(" "));

    array
}
