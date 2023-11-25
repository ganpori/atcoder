fn a() {
    input! {
      a:usize,
      b:usize,
    }
    if b - a == 1 && b % 3 != 1 {
        println!("Yes");
        return;
    }

    println!("No",);
}

fn b() {
    input! {
      n:usize,
      a:[Chars;n]
    }

    // 一行目
    print!("{}", a[1][0]);
    for i in 0..n - 1 {
        print!("{}", a[0][i]);
    }
    println!("");

    //間
    for i in 1..n - 1 {
        print!("{}", a[i + 1][0]);
        for j in 1..n - 1 {
            print!("{}", a[i][j]);
        }
        print!("{}\n", a[i - 1][n - 1]);
    }

    // 最終行
    for i in 1..n {
        print!("{}", a[n - 1][i]);
    }
    print!("{}\n", a[n - 2][n - 1]);
}

// vecの要素がvecの時、中身のvecの値でsort:ab.sort_by(|a, b| a[0].cmp(&b[0]));
//　境界のややこしい話をできる限りデータ構造に押し込める。さもないと謎の境界エラーに苦しめられる。
fn c() {
    input! {
      n:usize,
      k:usize,
    mut ab:[[usize;2];n]
    }

    ab.sort_by(|a, b| a[0].cmp(&b[0]));
    // 日付に対して飲む数が単調減少なので二分探索が可能。
    // 累積和のほうが良い？
    // そもそも探索せずとも関数作れる？
    // 変化点だけ検知すればよい
    let mut num_medicine = HashMap::new();
    let mut sum = 0;
    for i in 0..n {
        *num_medicine.entry(ab[i][0]).or_insert(0) += ab[i][1];
        sum += ab[i][1];
    }
    if sum <= k {
        println!("1");
        return;
    }

    for day in num_medicine.keys().sorted() {
        // day+1日からnum_medicine[key]個飲む薬が減る
        sum -= num_medicine[day];
        if sum <= k {
            println!("{}", day + 1);
            return;
        }
    }
}

fn d() {
    input! {
      n1:usize,
      n2:usize,
      m:usize,
      ab:[[usize;2];m]
    }
    // 二つのグラフの長さを足せばよい
    let mut g = vec![Vec::new(); n1 + n2];
    for i in 0..m {
        g[ab[i][0] - 1].push(ab[i][1] - 1);
        g[ab[i][1] - 1].push(ab[i][0] - 1);
    }

    let mut queue_node = VecDeque::new();

    // 0から探索
    let mut node = 0;
    let mut dist = vec![-1; n1 + n2]; //distの値が訪問済みの判定にも使える
    dist[node] = 0;
    queue_node.push_back(node);
    while queue_node.len() > 0 {
        node = queue_node.pop_front().unwrap();
        for next_node in &g[node] {
            if dist[*next_node] == -1 {
                queue_node.push_back(*next_node);
                dist[*next_node] = dist[node] + 1;
            } else {
                continue;
            }
        }
    }
    let max_n1 = dist.iter().max().unwrap();

    // n1+n2-1から探索
    let mut node = n1 + n2 - 1;
    let mut dist = vec![-1; n1 + n2]; //distの値が訪問済みの判定にも使える
    dist[node] = 0;
    queue_node.push_back(node);
    while queue_node.len() > 0 {
        node = queue_node.pop_front().unwrap();
        for next_node in &g[node] {
            if dist[*next_node] == -1 {
                queue_node.push_back(*next_node);
                dist[*next_node] = dist[node] + 1;
            } else {
                continue;
            }
        }
    }
    let max_n2 = dist.iter().max().unwrap();

    println!("{}", max_n1 + max_n2 + 1);
}
