#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      s:Chars,
    }

    let c1 = s[0];
    if !c1.is_ascii_uppercase() {
        println!("No");
        return;
    }
    for i in 1..s.len() {
        if !s[i].is_lowercase() {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
