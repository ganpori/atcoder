fn a() {
    input! {
      b:usize,
      g:usize
    }
    if b > g {
        println!("Bat");
    } else {
        println!("Glove");
    }
}

fn d() {
    input! {
      n:usize,
      q:usize,
     mut r:[usize;n]
    }
    r.sort();

    let mut sum_r = vec![0; n];
    sum_r[0] = r[0];
    for i in 1..n {
        sum_r[i] = sum_r[i - 1] + r[i];
    }

    for _ in 0..q {
        input! {x:usize}
        let i = sum_r.partition_point(|&a| a <= x);
        println!("{}", i);
    }
}
