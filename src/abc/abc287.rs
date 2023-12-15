fn a() {
    input! {
      n:usize,
      s:[Chars;n]
    }
    let mut num_f = 0;
    for i in 0..n {
        if s[i][0] == 'F' {
            num_f += 1;
        }
    }
    if num_f >= n / 2 + 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn b() {
    input! {
      n:usize,
      m:usize,
      s:[Chars;n],
      t:[Chars;m]
    }
    let mut count = 0;
    for si in s {
        for ti in t.iter() {
            if si[3] == ti[0] && si[4] == ti[1] && si[5] == ti[2] {
                count += 1;
                break;
            }
        }
    }
    print!("{}", count);
}

fn c() {
    input! {
      n:usize,
      m:usize,
      uv:[[usize;2];m]
    }

    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[uv[i][0] - 1].push(uv[i][1] - 1);
        g[uv[i][1] - 1].push(uv[i][0] - 1);
    }
    let mut num1 = 0;
    let mut num2 = 0;
    for edge in &g {
        if edge.len() == 1 {
            num1 += 1
        } else if edge.len() == 2 {
            num2 += 1;
        }
    }
    if num1 != 2 || num2 != n - 2 {
        print!("No");
        return;
    }

    let mut q = VecDeque::new();
    let start_pos = 0;
    let mut dist = vec![-1; n];
    q.push_back(start_pos);
    dist[start_pos] = 0;
    while q.len() > 0 {
        let pos = q.pop_front().unwrap();
        for next_pos in &g[pos] {
            if dist[*next_pos] == -1 {
                q.push_back(*next_pos);
                dist[*next_pos] = dist[pos] + 1;
            }
        }
    }
    for d in dist {
        if d == -1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

// 事前にマッチしてるかを計算しておけば重複した計算しなくて済む
fn d() {
    input! {
      s:Chars,
      t:Chars
    }

    let mut prefix = vec![false; s.len()];
    if t[0] == s[0] || t[0] == '?' || s[0] == '?' {
        prefix[0] = true;
    }
    for i in 1..t.len() {
        if prefix[i - 1] && (t[i] == s[i] || t[i] == '?' || s[i] == '?') {
            prefix[i] = true;
        }
    }

    let mut suffix = vec![false; s.len()];
    if t[t.len() - 1] == s[s.len() - 1] || t[t.len() - 1] == '?' || s[s.len() - 1] == '?' {
        suffix[s.len() - 1] = true;
    }
    for i in 1..t.len() {
        if suffix[s.len() - i]
            && (t[t.len() - 1 - i] == s[s.len() - 1 - i]
                || t[t.len() - 1 - i] == '?'
                || s[s.len() - 1 - i] == '?')
        {
            suffix[s.len() - 1 - i] = true;
        }
    }
    // dbg!(&prefix, &suffix);

    for x in 0..=t.len() {
        let is_same: bool;
        if x == 0 {
            is_same = suffix[s.len() - t.len()];
        } else if x == t.len() {
            is_same = prefix[x - 1];
        } else {
            is_same = prefix[x] && suffix[s.len() - (t.len() - x)];
        }
        if is_same {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
