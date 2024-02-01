#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      s:Chars
    }
    for i in 1..n {
        let mut length = 0;

        for k in 1..=n - i {
            // dbg!(k - 1, k + i - 1);
            if s[k - 1] == s[k + i - 1] {
                length = k - 1;
                break;
            }
            if k + i == n {
                length = k;
            }
        }
        println!("{}", length);
    }
}
