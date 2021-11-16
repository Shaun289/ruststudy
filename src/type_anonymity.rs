/*
The rust by example ko https://hanbum.gitbooks.io/rustbyexample/content/fn/closures/anonymity.html
compiled on https://play.rust-lang.org/
Type anonymity : 익명성....???
result :
*/

fn apply<F> (f: F) where
    F: Fn()
{
    f()
}

fn main()
{
    let x = 7;

    let print = || println!("{}", x);
    apply(print);
}
