fn a() {
    input! {
      n:usize,
      p:[usize;n]
    }

    let max_p = p.iter().max().unwrap();
    let mut count_max = 0;
    for i in 0..n {
        if *max_p == p[i] {
            count_max += 1;
        }
    }
    if *max_p == p[0] && count_max == 1 {
        print!("0");
    } else {
        println!("{}", max_p - p[0] + 1);
    }
}
