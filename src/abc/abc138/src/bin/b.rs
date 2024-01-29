#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      a:[f64;n]
    }

    let mut sum = 0.0;
    for i in 0..n {
        sum += 1.0 / a[i];
    }
    let result = 1.0 / sum;
    println!("{}", result);
}
