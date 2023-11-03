// https://atcoder.jp/contests/abc326
use proconio::input;

pub fn a() {
    input! {n:usize,}
}

pub fn b() {
    input! {n:usize,}
}

pub fn c() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n]
    }

    a.sort();

    let mut right = vec![0; n];
    let mut num_present_max = 0;

    for (i, ai) in a.iter().enumerate() {
        if (i != 0) {
            right[i] = std::cmp::max(right[i - 1], i);
        }
        let j_add = right[i];
        for (j, aj) in a[right[i]..].iter().enumerate() {
            let j_in_a = j + j_add;
            // println!("j_in_a:{}", j_in_a);
            if (*ai + m > *aj) {
                num_present_max = std::cmp::max(num_present_max, j_in_a - i + 1);
                right[i] = j_in_a;
            } else {
                break;
            }
        }
    }
    // println!("{:?}", a);
    println!("{}", num_present_max);
}

pub fn d() {
    input! {n:usize,}
}
