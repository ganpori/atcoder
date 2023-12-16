fn a() {
    input! {
      n:usize,
    }
    for i in 0..n {
        print!("{}", n);
    }
}

// 文字列の判定もうちょっと楽にする方法ないの？
fn b() {
    input! {
      mut s:Chars,
      mut t:Chars
    }
    s.sort();
    t.sort();

    let mut s_dist: usize = 0;
    let mut t_dist: usize = 0;

    let dist = [
        ("AB", 1),
        ("AC", 2),
        ("AD", 2),
        ("AE", 1),
        ("BC", 1),
        ("BD", 2),
        ("BE", 2),
        ("CD", 1),
        ("CE", 2),
        ("DE", 1),
    ];
    for d in dist {
        let mozi = d.0;
        let mut iter = mozi.chars();
        let mozi1 = iter.next().unwrap();
        let mozi2 = iter.next().unwrap();
        if mozi1 == s[0] && mozi2 == s[1] {
            s_dist = d.1;
        }
        if mozi1 == t[0] && mozi2 == t[1] {
            t_dist = d.1;
        }
    }
    if t_dist == s_dist {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 問題読まずに時間ロスした。
// 先ずは例をちゃんと確認する。
fn c() {
    input! {
      n:usize,
    }

    let mut memo: Vec<usize> = vec![
        1,
        11,
        111,
        1_111,
        11_111,
        111_111,
        1_111_111,
        11_111_111,
        111_111_111,
        1_111_111_111,
        11_111_111_111,
        111_111_111_111,
    ];
    let mut sum: Vec<usize> = vec![];
    for c in memo.iter().combinations_with_replacement(3) {
        sum.push(c.into_iter().sum());
    }
    sum.sort();
    println!("{}", sum[n - 1]);
}

// 一つ点を除いてunionfindすれば枝を分割できた。
fn d() {
    input! {
      n:usize,
      uv:[[usize;2];n-1]
    }

    let mut adjacency_list = vec![Vec::<usize>::new(); n];
    for edge in uv.iter() {
        adjacency_list[edge[0] - 1].push(edge[1] - 1);
        adjacency_list[edge[1] - 1].push(edge[0] - 1);
    }
    let mut dist: Vec<isize> = vec![-1; n];
    dist[0] = 0;

    let mut queue = VecDeque::new();
    let mut max_num_node: usize = 0;
    for node in &adjacency_list[0] {
        let mut num_node = 1;
        dist[*node] = 1;
        queue.push_back(node);

        while queue.len() > 0 {
            let pos = queue.pop_front().unwrap();
            for to in &adjacency_list[*pos] {
                if dist[*to] == -1 {
                    queue.push_back(to);
                    dist[*to] = dist[*pos] + 1;
                    num_node += 1;
                }
            }
        }

        max_num_node = std::cmp::max(max_num_node, num_node);
    }
    if adjacency_list[0].len() == 1 {
        println!("1");
    } else {
        println!("{}", n - max_num_node);
    }
}
