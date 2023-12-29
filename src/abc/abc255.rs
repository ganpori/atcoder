fn a() {
    input! {
        r:usize,
        c:usize,
        a:[[usize;2];2]
    }
    println!("{}", a[r - 1][c - 1]);
}

fn c() {
    input! {
      x:isize,
      a:isize,
      d:isize,
      n:isize
    }
    let mut num_last = a + (n - 1) * d;
    let num_min;
    let num_max;
    if d > 0 {
        num_min = a;
        num_max = num_last;
    } else if d < 0 {
        num_max = a;
        num_min = num_last;
    } else {
        num_min = a;
        num_max = a;
    }
    let mut diff: isize = 0;
    if x >= num_max {
        diff = x - num_max;
    } else if x <= num_min {
        diff = num_min - x;
    } else {
        let quotient = (x - a) / d;
        diff = std::cmp::min(
            ((x - a) - quotient * d).abs(),
            ((x - a) - (quotient + 1) * d).abs(),
        );
        diff = std::cmp::min(diff, ((x - a) - (quotient - 1) * d).abs());
    }

    println!("{}", diff);
}
