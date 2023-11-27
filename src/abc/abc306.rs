fn a() {
    input! {
        n:usize,
      s:Chars,
    }

    for c in s {
        print!("{}{}", c, c);
    }
}

fn b() {
    input! {
      a:[usize;64],
    }

    let mut sum = 0;
    for i in 0..64 {
        sum += a[i] * 2_usize.pow(i as u32);
    }
    println!("{}", sum);
}

fn c() {
    input! {
      n:usize,
      a:[usize;3*n]
    }

    let mut map_count = HashMap::new();
    let mut f = vec![[0, 0]; n];
    for i in 0..3 * n {
        let value = a[i];
        *map_count.entry(value).or_insert(0) += 1;
        if map_count[&value] == 2 {
            f[value - 1] = [i, value];
        }
    }

    f.sort();
    for i in 0..n {
        print!("{} ", f[i][1]);
    }
}

//　std::isize::MINで初期化しちゃうとさらにマイナスを足す場合オーバーフローしちゃう。
//  dpの書くマスはその時の状態を保持している
fn d() {
    input! {
      n:usize,
      xy:[[isize;2];n]
    }

    //　値は幸せの総和,01は1が毒状態になっているが得られる総量、0は毒じゃない状態で得られる総量
    let mut dp = vec![vec![-1; 2]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let doku = xy[i][0];
        if doku == 1 {
            dp[i + 1][0] = dp[i][0];
            dp[i + 1][1] = std::cmp::max(dp[i][0] + xy[i][1], dp[i][1]);
        } else if doku == 0 {
            // まずいものがあるから食わないほうがいいときもある
            dp[i + 1][0] = std::cmp::max(dp[i][0] + xy[i][1], dp[i][1] + xy[i][1]); // 食べた場合の最大値
            dp[i + 1][0] = std::cmp::max(dp[i + 1][0], dp[i][0]); // 食べない場合の最大値
            dp[i + 1][1] = dp[i][1];
        }
        // dbg!(&dp);
    }

    println!("{}", std::cmp::max(dp[n][0], dp[n][1]));
}
