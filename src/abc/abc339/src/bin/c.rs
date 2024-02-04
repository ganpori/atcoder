#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      a:[isize;n]
    }
    let mut now = 0;

    let mut min_val = isize::MAX;
    for i in 0..n {
        now += a[i];
        min_val = std::cmp::min(min_val, now);
    }
    if min_val < 0 {
        now = now - min_val;
    }
    println!("{}", now);
}
