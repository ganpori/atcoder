#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
    mut v:[usize;n]
    }
    v.sort();

    let mut value = v[0] as f64;
    for i in 1..n {
        value = (value + v[i] as f64) / 2.0;
    }
    println!("{}", value);
}
