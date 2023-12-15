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

fn b17() {
    input! {
      n:usize,
      h:[isize;n],

    }
    let mut dp = vec![100 * 100_000 + 1; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    for i in 2..n {
        dp[i] = std::cmp::min(
            dp[i - 1] + (h[i] - h[i - 1]).abs(),
            dp[i - 2] + (h[i] - h[i - 2]).abs(),
        );
    }

    let mut trajectory: Vec<usize> = vec![];
    let mut pos = n - 1;
    trajectory.push(pos + 1);
    while pos > 1 {
        if dp[pos] == dp[pos - 1] + (h[pos - 1] - h[pos]).abs() {
            pos -= 1;
        } else {
            pos -= 2;
        }
        trajectory.push(pos + 1);
    }
    if pos == 1 {
        trajectory.push(1);
    }
    trajectory.sort();
    println!("{}", trajectory.len());
    for i in 0..trajectory.len() {
        print!("{} ", trajectory[i]);
    }
}

// 横が獲得点数、縦がどのカードを使うか
// dpの中身は使う使わないのbool
// 獲得点数が0,使うカード枚数が0の行と列を一つずつ追加すると見通しが立つ
// 使える枚数は一つなんだから同じ行を参照してはいけない
fn a18() {
    input! {
      n:usize,
      s:usize,
      a:[usize;n]
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..n + 1 {
        for j in 0..s + 1 {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
            if a[i - 1] <= j {
                let target_j = j - a[i - 1];
                if dp[i - 1][target_j] {
                    dp[i][j] = true;
                }
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
    }
    // dbg!(&dp);
}

fn b18() {
    input! {
      n:usize,
      s:usize,
      a:[usize;n]
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..n + 1 {
        for j in 0..s + 1 {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
            if a[i - 1] <= j {
                let target_j = j - a[i - 1];
                if dp[i - 1][target_j] {
                    dp[i][j] = true;
                }
            }
        }
    }
    if dp[n][s] {
        // println!("Yes");
        let mut trajectory: Vec<usize> = vec![];
        let mut card = n;
        let mut value = s;
        while value > 0 {
            if value >= a[card - 1] {
                if dp[card - 1][value - a[card - 1]] {
                    trajectory.push(card);
                    value -= a[card - 1];
                }
            }
            card -= 1;
        }
        println!("{}", trajectory.len());
        trajectory.sort();
        for card in trajectory {
            print!("{} ", card);
        }
    } else {
        println!("-1");
    }
    // dbg!(&dp);
}

fn a19() {
    input! {
      n:usize,
      w:usize,
      wv:[[usize;2];n]
    }
    let mut dp: Vec<Vec<isize>> = vec![vec![-1; w + 1]; n + 1];
    dp[0][0] = 0;
    let mut max_value = 0;
    for i in 1..n + 1 {
        for j in 0..w + 1 {
            let weight_target = wv[i - 1][0];
            let value_target = wv[i - 1][1];
            if dp[i - 1][j] != -1 {
                dp[i][j] = dp[i - 1][j];
            }
            if j >= weight_target {
                if dp[i - 1][j - weight_target] != -1 {
                    dp[i][j] = std::cmp::max(
                        dp[i][j],
                        dp[i - 1][j - weight_target] + value_target as isize,
                    );
                }
            }
            max_value = std::cmp::max(max_value, dp[i][j]);
        }
    }
    // dbg!(&dp);
    println!("{}", max_value);
}

fn b19() {
    input! {
      n:usize,
      w:usize,
      wv:[[usize;2];n]
    }

    let value_max = n * 1000;
    let mut dp: Vec<Vec<usize>> = vec![vec![w + 1; value_max + 1]; n + 1];
    dp[0][0] = 0;
    let mut max_value = 0;
    for i in 1..n + 1 {
        for j in 0..value_max + 1 {
            let weight_target = wv[i - 1][0];
            let value_target = wv[i - 1][1];
            if dp[i - 1][j] != w + 1 {
                dp[i][j] = dp[i - 1][j];
                if dp[i][j] <= w {
                    max_value = std::cmp::max(max_value, j);
                }
            }
            if j >= value_target {
                if dp[i - 1][j - value_target] != w + 1 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j - value_target] + weight_target);
                    if dp[i][j] <= w {
                        max_value = std::cmp::max(max_value, j);
                    }
                }
            }
        }
    }
    // dbg!(&dp);
    println!("{}", max_value);
}
