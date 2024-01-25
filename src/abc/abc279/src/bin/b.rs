#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      s:Chars,
      t:Chars
    }
    if s.len() < t.len() {
        println!("No");
        return;
    }
    if s.len() == t.len() {
        let mut is_partial = true;
        for i in 0..t.len() {
            if s[i] != t[i] {
                is_partial = false;
            }
        }
        if is_partial {
            println!("Yes");
            return;
        }
    }

    for i in 0..s.len() + 1 - t.len() {
        let mut is_partial = true;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                is_partial = false;
            }
        }
        if is_partial {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
