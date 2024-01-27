use nalgebra::min;
#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      a:[usize;n]
    }
    let mut l = vec![1; n];

    for i in 1..n {
        l[i] = std::cmp::min(a[i], l[i - 1] + 1);
    }

    let mut r = vec![1; n];
    for i in 1..n {
        r[n - i - 1] = min(a[n - i - 1], r[n - i] + 1);
    }

    let mut max_p = 1;

    for i in 0..n {
        max_p = std::cmp::max(max_p, min(l[i], r[i]));
    }
    println!("{}", max_p);
    // dbg!(r, l);
}
