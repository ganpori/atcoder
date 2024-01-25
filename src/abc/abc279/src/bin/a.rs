#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      s:Chars,
    }

    let mut count = 0;
    for c in s {
        if c == 'v' {
            count += 1;
        } else if c == 'w' {
            count += 2;
        }
    }
    println!("{}", count);
}
