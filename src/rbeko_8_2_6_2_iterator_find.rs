/*
The rust by example ko https://hanbum.gitbooks.io/rustbyexample/content/fn/closures/closure_examples/iter_find.html
compiled on https://play.rust-lang.org/
*/
/*
pub trait Iterator {
    // 반복되는 타입
    type Item;
    
    // find는 & mut self 를 취하며 그 의미는 호출자가 대여하거나
    // 수정할 수는 있어도 소비되지 않는다.
    fn find<P>(&mut self, predicate:P) -> Option<Self::Item> where
        // FnMut의 의미는 모든 캡쳐된 변수가 대부분 수정될 수 있고 소비되지는 않는다.
        // &Self::Item은 클로져에 대한 인수를 참조로 받는다.
        P: FnMut(&Self::Item) -> bool {}
}
*/

fn main() {
    let vec1 = vec!{1, 2, 3};
    let vec2 = vec!{4, 5, 6};
    
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();
    
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {:?} ", array1.iter().find(|&&x| x == 2));
    // 컴파일이 안되는데..?
    //println!("2 in array1: {} ", array2.into_iter().find(|&&x| x == 2));
    
}
