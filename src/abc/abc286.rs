fn a() {
    input! {
      n:usize,
      p:usize,
      q:usize,
      r:usize,
      s:usize,
      a:[usize;n]
    }

    for i in 0..n {
        if p <= i + 1 && i + 1 <= q {
            print!("{} ", a[r + i - p]);
        } else if r <= i + 1 && i + 1 <= s {
            print!("{} ", a[p + i - r]);
        } else {
            print!("{} ", a[i]);
        }
    }
}

fn b() {
    input! {
      n:usize,
      s:Chars
    }

    for i in 0..n - 1 {
        print!("{}", s[i]);
        if s[i] == 'n' && s[i + 1] == 'a' {
            print!("y");
        }
    }
    print!("{}", s.last().unwrap());
}

fn c() {
    input! {
      n:usize,
      a: usize,
      b:usize,
      s:Chars
    }

    let mut min_cost = n * b + n * a;
    for num_rotate in 0..n - 1 {
        let mut count_same = 0;
        for i in 0..n {
            let from_back = n - 1 - i;
            let index = (from_back + 2 * num_rotate) % n;
            if s[i] == s[index] {
                count_same += 1;
            }
        }
        let count_wrong = n - count_same;
        min_cost = std::cmp::min(min_cost, count_wrong / 2 * b + num_rotate * a);
    }
    println!("{}", min_cost);
    // 1: n,2,4,6,8,..,n-2,n
    // 2: n-1,1,3,5
}

fn d() {
    input! {
      n:usize,
      x:usize,
      ab:[[usize;2];n]
    }

    let mut num_coins = 0;
    let mut vec_value = vec![];
    for i in 0..n {
        num_coins += ab[i][1];
        for j in 0..ab[i][1] {
            vec_value.push(ab[i][0]);
        }
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; 10001]; num_coins + 1];

    // 0円効果を使ったとき。1枚で0円を達成できる
    dp[0][0] = true;

    // dbg!(&vec_value);
    for i in 1..=num_coins {
        let value = vec_value[i - 1];
        for j in 0..=10000 {
            if j as i32 - value as i32 >= 0 {
                if dp[i - 1][j - value] == true {
                    dp[i][j] = true;
                }
            }
            if dp[i - 1][j] == true {
                dp[i][j] = true;
            }
        }
    }

    if dp[num_coins][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
