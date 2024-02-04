#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      s:Chars,
    }

    let mut index = 0;
    for i in 0..s.len() {
        if s[s.len() - 1 - i] == '.' {
            index = s.len() - i;
            break;
        }
    }

    for i in index..s.len() {
        print!("{}", s[i]);
    }
}
