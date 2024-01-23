#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      s:Chars
    }

    let mut is_wrapped = false;
    for c in s {
        if c == '"' {
            is_wrapped = !is_wrapped;
        }
        if !is_wrapped && c == ',' {
            print!(".");
        } else {
            print!("{}", c);
        }
    }
}
