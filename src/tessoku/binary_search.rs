// モジュールの概念の説明
// https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
// https://zenn.dev/newgyu/articles/3b4677b4086768

use proconio::input;

pub fn a11() {
    input! {
        n:usize,
        x:usize,
        a:[usize;n]
    }

    let result = a.binary_search(&x);
    let index = result.unwrap();
    println!("{}", index + 1);
    // let mut index_max = a.len() - 1;
    // let mut index_min = 0usize;
    // let mut half_index;
    // loop {
    //     half_index = (index_min + index_max) / 2;
    //     if a[half_index] < x {
    //         index_min = half_index + 1;
    //     } else if a[half_index] > x {
    //         index_max = half_index - 1;
    //     } else {
    //         println!("{}", half_index + 1);
    //         break;
    //     }
    // }
}

pub fn a12() {
    input! {
        n:usize,
        k:usize,
        mut a:[usize;n]
    }
    a.sort();

    let mut max_time = a[0] * k;
    let mut min_time = 1;

    loop {
        let mut time_candidate = (max_time + min_time) / 2;
        let mut num_print = 0;
        for ai in a.iter() {
            num_print += time_candidate / ai;
        }

        if num_print >= k {
            // num_print==kでも同じnum_printのところが何秒か持続する。そのうちの最小を探すためにmaxを小さくしていく
            max_time = time_candidate; // maxtimeもずらすけどcandidateより小さくすると間違う可能性あり
            if max_time == min_time {
                println!("{}", time_candidate);
                break;
            }
        } else if num_print < k {
            min_time = time_candidate + 1;
        }
        // println!("num_print:{}", num_print);
        // println!("time_candidate:{}", time_candidate);
        // println!("num_print:{}", num_print);
    }
}

pub fn a13() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n]
    }

    let mut r = vec![0; n]; //右端のindex, 右端がわかってればその間は計算する必要がなくなるから。
    for (i, ai) in a[..a.len() - 1].iter().enumerate() {
        // println!("{}", ai);
        if (i == 0) {
        } else {
            r[i] = r[i - 1];
        }
        for aj in &a[r[i] + 1..] {
            // println!("aj:{}", aj);
            if k >= (aj - ai) {
                r[i] += 1;
            } else {
                break;
            }
        }
        // println!("r:{:?}", r);
    }
    let mut count = 0;
    for (i, ri) in r[..r.len() - 1].iter().enumerate() {
        count += ri - i;
    }
    println!("{}", count);
}
