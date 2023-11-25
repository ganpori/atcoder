fn a() {
    input! {
      n:usize,
      l:usize,
      a:[usize;n]
    }

    let mut num = 0;
    for i in 0..n {
        if a[i] >= l {
            num += 1;
        }
    }

    println!("{}", num);
}

fn b() {
    input! {
      n:usize,
      l:usize,
      r:usize,
      a:[usize;n]
    }

    for i in 0..n {
        let mut x = a[i];
        if a[i] < l {
            x = l;
        } else if a[i] > r {
            x = r;
        }

        print!("{} ", x);
    }
}

//　適切な範囲で全探索する
fn c() {
    input! {
      d:usize,
    }
    // x <=y　とする。yを雑に決める。xを細かく決める。

    let sqrt_d_int = (d as f64).sqrt() as usize;

    let y_lower = sqrt_d_int;
    let y_upper = sqrt_d_int + 1;
    let y_lower_pow = y_lower * y_lower;
    let y_upper_pow = y_upper * y_upper;

    // res-x^2を最小にするxを求める
    // そのxを使うかy_upper_powをつかうかの二択
    // 全探索で行けそう
    let mut min_abs: usize = d;
    for y in 1..=y_lower {
        let res = (d - y * y) as isize;
        let res_sqrt = (res as f64).sqrt() as usize;
        let mut old_abs = res as usize;
        for x in res_sqrt..=y {
            let abs = (res - (x * x) as isize).abs() as usize;
            min_abs = std::cmp::min(abs, min_abs);
            if min_abs == 0 {
                println!("0");
                return;
            }
            if abs > old_abs {
                break;
            }
            old_abs = abs;
        }
    }
    min_abs = std::cmp::min(y_upper_pow - d, min_abs);

    // for i in 0..n {}

    println!("{}", min_abs);
}
