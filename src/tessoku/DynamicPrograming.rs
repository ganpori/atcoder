fn a16() {
    input! {
      n:usize,
      a:[usize;n-1],
      b:[usize;n-2]
    }
    let mut dp = vec![100 * 100_000 + 1; n];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = std::cmp::min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }
    println!("{}", dp[n - 1]);
}

fn a17() {
    input! {
      n:usize,
      a:[usize;n-1],
      b:[usize;n-2]
    }
    let mut dp = vec![100 * 100_000 + 1; n];
    dp[0] = 0;
    dp[1] = a[0];
    for i in 2..n {
        dp[i] = std::cmp::min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }

    let mut trajectory: Vec<usize> = vec![];
    let mut pos = n - 1;
    trajectory.push(pos + 1);
    while pos > 1 {
        // dbg!(
        //     pos,
        //     dp[pos],
        //     dp[pos - 1],
        //     a[pos - 2],
        //     dp[pos - 1] + a[pos - 2]
        // );

        if dp[pos] == dp[pos - 1] + a[pos - 1] {
            pos -= 1;
        } else {
            pos -= 2;
        }
        trajectory.push(pos + 1);
    }
    if pos == 1 {
        trajectory.push(1);
    }
    // dbg!(&dp);
    println!("{}", trajectory.len());
    for i in 0..trajectory.len() {
        print!("{} ", trajectory[trajectory.len() - 1 - i]);
    }
}
