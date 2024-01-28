#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      q:[usize;n],
      a:[usize;n],
      b:[usize;n]
    }

    let q_max = q.iter().max().unwrap();

    let mut max_num = 0;
    for num_a in 0..=*q_max {
        let mut exist_num_a = true;
        let mut num_b = q_max + 1;
        for i in 0..n {
            if q[i] < a[i] * num_a {
                exist_num_a = false;
            } else {
                if b[i] > 0 {
                    num_b = std::cmp::min((q[i] - a[i] * num_a) / b[i], num_b);
                }
            }
        }
        if exist_num_a {
            max_num = std::cmp::max(max_num, num_a + num_b);
        }
    }
    println!("{}", max_num);
}
