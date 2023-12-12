fn a() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:[usize;n]
    }
    for i in 0..n {
        if a + b == c[i] {
            println!("{}", i + 1);
        }
    }
}

fn b() {
    input! {
        h:usize,
        w:usize,
        a:[Chars;h],
        b:[Chars;h]

    }

    for hs in 0..=h - 1 {
        for ws in 0..=w - 1 {
            let mut is_same = true;
            for i in 0..h {
                for j in 0..w {
                    if a[(i + hs) % h][(j + ws) % w] != b[i][j] {
                        is_same = false;
                    }
                }
            }
            if is_same {
                print!("Yes");
                return;
            }
        }
    }
    print!("No");
}

fn c() {
    input! {
        h:isize,
        w:isize,
        mut c:[Chars;h]
    }
    let n = std::cmp::min(h, w);
    let mut size_map = HashMap::new();
    for i in 0..=n {
        size_map.insert(i, 0);
    }

    let dh = [-1, -1, 1, 1];
    let dw = [1, -1, -1, 1];

    let mut q = VecDeque::new();

    for i in 0..h {
        for j in 0..w {
            let mut dist = vec![vec![-1; w as usize]; h as usize];
            let start_pos: [isize; 2] = [i, j];
            dist[start_pos[0] as usize][start_pos[1] as usize] = 0;
            q.push_back(start_pos);

            let mut max_dist = 0;
            while q.len() > 0 {
                let pos = q.pop_front().unwrap();
                c[pos[0] as usize][pos[1] as usize] = '.';
                for i in 0..4 {
                    let next_pos = [pos[0] + dh[i], pos[1] + dw[i]];
                    if 0 <= next_pos[0]
                        && next_pos[0] <= h - 1
                        && 0 <= next_pos[1]
                        && next_pos[1] <= w - 1
                    {
                        if c[next_pos[0] as usize][next_pos[1] as usize] == '#'
                            && dist[next_pos[0] as usize][next_pos[1] as usize] == -1
                        {
                            dist[next_pos[0] as usize][next_pos[1] as usize] =
                                1 + dist[pos[0] as usize][pos[1] as usize];
                            q.push_back(next_pos);
                            max_dist = 1 + dist[pos[0] as usize][pos[1] as usize];
                        }
                    }
                }
            }
            let size = max_dist / 2;
            *size_map.entry(size).or_insert(0) += 1;
            // dbg!(&size, &size_map);
        }
    }
    for i in 1..=n {
        print!("{} ", size_map[&i]);
    }
}
