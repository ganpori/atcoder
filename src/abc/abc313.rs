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

// 有向グラフで力関係を表現。このグラフは木になる。
// 最強なら木の根になって全部のノードに探索可能。
// シミュレーションでまけたやつを候補から外していくというの解放単純。
fn b() {
    input! {
      n:usize,
      m:usize,
      ab:[[usize;2];m],
    }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..m {
        g[ab[i][0] - 1].push(ab[i][1] - 1);
    }

    // println!("{:?}", g);
    let mut saikyo_man = n + 1;
    for i in 0..n {
        let mut visited = vec![false; n];
        dfs(i, &mut visited, &g);
        if visited.iter().all(|&x| x) {
            saikyo_man = i;
            break;
        }
    }
    if saikyo_man == n + 1 {
        println!("-1",);
    } else {
        println!("{}", saikyo_man + 1);
    }
}

fn dfs(pos: usize, visited: &mut Vec<bool>, g: &Vec<Vec<usize>>) {
    visited[pos] = true;
    for next_pos in &g[pos] {
        if visited[*next_pos] {
            continue;
        } else {
            dfs(*next_pos, visited, g);
        }
    }
}
