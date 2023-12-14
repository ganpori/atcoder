fn a() {
    input! {
      s:Chars,
    }
    for c in s.iter() {
        let mut a = c.clone();
        a.make_ascii_uppercase();
        print!("{}", a);
    }
}

fn b() {
    input! {
      n:usize,
      q:usize
    }

    let mut map_card: HashMap<usize, usize> = (1..=n).map(|key| (key, 0)).collect();
    for _ in 0..q {
        input! {
            c:usize,
            x:usize
        }
        if c == 1 {
            map_card.insert(x, map_card[&x] + 1);
        } else if c == 2 {
            map_card.insert(x, map_card[&x] + 2);
        } else {
            if map_card[&x] >= 2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

// 二つの組の大小も仮定するとsqrtまで調べるだけで十分になる
fn c() {
    input! {
      n:usize,
    }

    let mut count = 0;
    if n % 2 == 1 {
        for cd in 1..=n / 2 {
            let ab = n - cd;
            let count_ab = count_comb(ab);
            let count_cd = count_comb(cd);
            count += count_ab * count_cd;
        }
        count = count * 2;
    } else {
        for cd in 1..=n / 2 - 1 {
            let ab = n - cd;
            let count_ab = count_comb(ab);
            let count_cd = count_comb(cd);
            count += count_ab * count_cd;
        }
        count = count * 2;
        count += count_comb(n / 2) * count_comb(n / 2);
    }

    println!("{}", count);
}

fn count_comb(n: usize) -> usize {
    let mut count_ab = 0;
    let n_sqrt = n.sqrt();
    for b in 1..=n_sqrt {
        let a = n / b;
        if a * b == n && a != b {
            count_ab += 1;
        }
    }
    count_ab *= 2;
    if n_sqrt * n_sqrt == n {
        count_ab += 1;
    }

    count_ab
}

// union findよく出てくるしやったほうがよさげ。
fn d() {
    input! {
      n:usize,
      m:usize,
      uv:[[usize;2];m]
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut set_self = HashSet::new();
    for i in 0..m {
        g[uv[i][0] - 1].push(uv[i][1] - 1);
        g[uv[i][1] - 1].push(uv[i][0] - 1);
        if uv[i][0] == uv[i][1] {
            set_self.insert(uv[i][0] - 1);
        }
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; n];

    for start_pos in 0..n {
        let mut trajectory: Vec<usize> = vec![];

        if !visited[start_pos] {
            trajectory.push(start_pos);
            q.push_back(start_pos);
            visited[start_pos] = true;
        }

        while q.len() > 0 {
            let pos = q.pop_front().unwrap();

            for next_pos in &g[pos] {
                if !visited[*next_pos] {
                    q.push_back(*next_pos);
                    visited[*next_pos] = true;
                    trajectory.push(*next_pos);
                }
            }
        }

        let mut sum_edge = 0;
        let mut num_self = 0;
        for trajectry_pos in trajectory.iter() {
            sum_edge += g[*trajectry_pos].len();
            if set_self.contains(trajectry_pos) {
                num_self += 1;
                sum_edge -= 1;
            }
        }
        sum_edge = sum_edge / 2 + num_self;
        if trajectory.len() != sum_edge {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
