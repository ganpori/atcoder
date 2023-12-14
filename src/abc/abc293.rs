fn a() {
    input! {
      mut s:Chars,
    }
    for i in 0..s.len() / 2 {
        s.swap(2 * i, 2 * i + 1)
    }

    for i in 0..s.len() {
        print!("{}", s[i]);
    }
}

fn b() {
    input! {
      n:usize,
      a:[usize;n]
    }

    let mut set_not_called: HashSet<usize> = (1..=n).into_iter().collect();
    for i in 0..n {
        let call = a[i];
        if set_not_called.contains(&(i + 1)) {
            set_not_called.remove(&call);
        }
    }

    let mut vec_not_called: Vec<usize> = set_not_called.into_iter().collect();
    vec_not_called.sort();
    println!("{}", vec_not_called.len());
    for i in 0..vec_not_called.len() {
        print!("{} ", vec_not_called[i]);
    }
}

fn c() {
    input! {
      h:usize,
      w:usize,
      a:[[usize;w];h]
    }

    let start_pos = [0, 0];
    let mut count = 0;
    let mut set_trajectory: HashSet<usize> = HashSet::new();
    set_trajectory.insert(a[0][0]);

    dfs(&start_pos, &a, &mut count, &mut set_trajectory);
    println!("{}", count);
}

fn dfs(
    pos: &[usize; 2],
    g: &Vec<Vec<usize>>,
    count: &mut usize,
    set_trajectory: &mut HashSet<usize>,
) {
    if pos[0] == g.len() - 1 && pos[1] == g[0].len() - 1 {
        *count += 1;
    }
    let dpos = [[0, 1], [1, 0]];
    let now_value = g[pos[0]][pos[1]];

    for d in dpos.iter() {
        let next_pos = [pos[0] + d[0], pos[1] + d[1]];
        if next_pos[0] <= g.len() - 1
            && next_pos[1] <= g[0].len() - 1
            && !set_trajectory.contains(&g[next_pos[0]][next_pos[1]])
        {
            set_trajectory.insert(g[next_pos[0]][next_pos[1]]);
            dfs(&next_pos, g, count, set_trajectory);
        }
    }
    set_trajectory.remove(&now_value);
}
