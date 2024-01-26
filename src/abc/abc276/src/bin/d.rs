#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      mut a:[usize;n]
    }

    let mut count2_all = 0;
    let mut count3_all = 0;
    let mut vec_a2 = vec![0; n];
    let mut vec_a3 = vec![0; n];
    for i in 0..n {
        let (c2, c3, pl) = get_count23_and_prime_like(a[i]);
        count2_all += c2;
        count3_all += c3;

        vec_a2[i] = c2;
        vec_a3[i] = c3;
        a[i] = pl;
    }

    if a.iter().all(|&x| x == a[0]) {
        let min_a2 = vec_a2.iter().min().unwrap();
        let min_a3 = vec_a3.iter().min().unwrap();
        let mut count = 0;
        for i in 0..n {
            count += vec_a2[i] - min_a2;
            count += vec_a3[i] - min_a3;
        }
        println!("{}", count);
    } else {
        println!("-1");
    }

    // dbg!(&a, count2_all, count3_all);
}
fn get_count23_and_prime_like(mut value: usize) -> (usize, usize, usize) {
    let mut count2 = 0;
    let mut count3 = 0;
    while value % 2 == 0 || value % 3 == 0 {
        if value % 2 == 0 {
            value /= 2;
            count2 += 1;
        }
        if value % 3 == 0 {
            value /= 3;
            count3 += 1;
        }
    }
    (count2, count3, value)
}
