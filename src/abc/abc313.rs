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

// 片方だけじゃなくて両方の側で最大値を判断しないといけない。
// どうしたらそう思えるだろう->
// ->手書き実験or経験則or怖いからそうしておくor論理で小さいほうの数と大きいほうの数が入れ替わることがあるときづく？
// 何か理想の形の数列があって、それに近づけていく、という着想も大事。
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
// 整数のオーバーフロー
// RustではDebugビルド時のみオーバーフロー、負のオーバーフローを実行時に検出します。
// オンライン上で実行されるときはReleaseビルドで実行されるのでオーバーフローは検出されないので注意しましょう。
// 64bitだとusizeは2^64 -1=18,446,744,073,709,551,615で19桁の整数までを表せる
fn c() {
    input! {
      n:usize,
      a:[usize;n]
    }

    let average_floor: usize = a.iter().sum::<usize>() / n;
    let average_upper;
    if a.iter().sum::<usize>() % n == 0 {
        average_upper = average_floor
    } else {
        average_upper = average_floor + 1;
    }
    let mut sum_diff_lower = 0;
    let mut sum_diff_upper = 0;
    for i in 0..n {
        if a[i] < average_floor {
            sum_diff_lower += average_floor - a[i];
        } else if a[i] >= average_upper {
            sum_diff_upper += a[i] - average_upper;
        }
    }
    println!("{}", std::cmp::max(sum_diff_lower, sum_diff_upper));
}
