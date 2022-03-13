mod baekjun_basic;

// do test :
// cargo test -- --show-output

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baekjun_basic() {
        assert!(baekjun_basic::baekjun_2557());
        assert!(baekjun_basic::baekjun_1330());
    }
}

