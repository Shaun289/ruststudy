/*
The rust by example ko https://hanbum.gitbooks.io/rustbyexample/content/fn/closures/closure_examples/iter_any.html
compiled on https://play.rust-lang.org/
*/
/*
pub trait Iterator {
    // 반복되는 타입
    type Item;
    
    // any가 취하는 &mut self 가 뜻하는 바는 호출자가 대여하거나 수정할지도 모르지만, 소모하지 않는다.
    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item) -> bool {}
}
*/

fn main() {
    let vec1 = vec!{1, 2, 3};
    let vec2 = vec!{4, 5, 6};
    
    // vec의 iter()는 &i32 를 산출한다. i32로 역구조화된다.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // vec의 into_iter() 는 i32를 산출한다. 역구조화가 필요치않다.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 배열의 iter() 는 &i32를 산출한다.
    println!("2 in array1: {} ", array1.iter().any(|&x| x == 2));
    // 배열의 into_iter() 는 &i32를 산출한다. 비일반적
    // 컴파일이 안되는데..?
    println!("2 in array1: {} ", array2.into_iter().any(|&x| x == 2));
    
}
