#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
     a:usize,
     b:usize
    }
    if a * 2 == b || a * 2 + 1 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
