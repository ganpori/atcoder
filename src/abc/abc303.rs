fn a() {
    input! {
      n:usize,
      s:Chars,
      t:Chars
    }

    for i in 0..n {
        let ss = s[i];
        let tt = t[i];
        if ss == tt {
        } else if ss == 'l' && tt == '1' || ss == '1' && tt == 'l' {
        } else if ss == 'o' && tt == '0' || ss == '0' && tt == 'o' {
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn b() {
    input! {
      n:usize,
      m:usize,
      a:[[usize;n];m]
    }

    let mut pair_set = HashSet::new();
    for j in 0..m {
        for i in 0..n - 1 {
            let mut pair = [&a[j][i], &a[j][i + 1]];
            pair.sort();
            pair_set.insert(pair);
        }
    }
    println!("{}", n * (n - 1) / 2 - pair_set.len());
}

//  xy:[[isize;2];m]の読み方はvec![vec![0,0,]]の型になるので注意
// 共通のitemまで抽出する必要はなかった。
fn c() {
    input! {
      n:usize,
      m:usize,
      mut h:isize,
      k:isize,
      s:Chars,
      xy:[[isize;2];m]
    }

    let mut trajectory = vec![vec![0, 0]; n + 1];
    for i in 1..=n {
        if s[i - 1] == 'R' {
            trajectory[i] = vec![trajectory[i - 1][0] + 1, trajectory[i - 1][1]];
        } else if s[i - 1] == 'L' {
            trajectory[i] = vec![trajectory[i - 1][0] - 1, trajectory[i - 1][1]];
        } else if s[i - 1] == 'U' {
            trajectory[i] = vec![trajectory[i - 1][0], trajectory[i - 1][1] + 1];
        } else if s[i - 1] == 'D' {
            trajectory[i] = vec![trajectory[i - 1][0], trajectory[i - 1][1] - 1];
        }
    }

    let trajectory_set: HashSet<&Vec<isize>> = trajectory.iter().collect();
    let medicine_set: HashSet<&Vec<isize>> = xy.iter().collect();
    let mut medicine_trajectory_set: HashSet<&Vec<isize>> = trajectory_set
        .intersection(&medicine_set)
        .cloned()
        .collect();

    // dbg!(
    //     &trajectory,
    //     &trajectory_set,
    //     &medicine_set,
    //     &medicine_trajectory_set
    // );
    let mut num = 0;
    while h >= 0 && num < n {
        let pos = &trajectory[num + 1];
        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }

        //item使う前に死んじゃう
        if h <= k && medicine_trajectory_set.contains(pos) {
            h = k;
            medicine_trajectory_set.remove(pos);
        }
        num += 1;
    }
    println!("Yes");
}

fn d() {
    input! {
      x:usize,
      y:usize,
      z:usize,
      s:Chars
    }
    let n = s.len();

    // 0がcaps_on,1がcaps_off状態で入力
    let mut dp = vec![[std::usize::MAX / 2; 2]; n];

    if s[0] == 'a' {
        dp[0][0] = x;
        dp[0][1] = z + y;
    } else if s[0] == 'A' {
        dp[0][0] = y;
        dp[0][1] = z + x;
    }

    for i in 1..n {
        if s[i] == 'a' {
            dp[i][0] = std::cmp::min(dp[i - 1][0] + x, dp[i - 1][1] + z + x);
            dp[i][1] = std::cmp::min(dp[i - 1][1] + y, dp[i - 1][0] + z + y);
        } else if s[i] == 'A' {
            dp[i][0] = std::cmp::min(dp[i - 1][0] + y, dp[i - 1][1] + z + y);
            dp[i][1] = std::cmp::min(dp[i - 1][1] + x, dp[i - 1][0] + z + x);
        }
    }
    let time = std::cmp::min(dp[n - 1][0], dp[n - 1][1]);
    println!("{}", time);
    // dbg!(dp);
}
