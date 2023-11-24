fn a() {
    input! {
        n:usize,
      s:Chars,
    }

    let mut has_A = false;
    let mut has_B = false;
    let mut has_C = false;
    for i in 0..s.len() {
        if s[i] == 'A' {
            has_A = true;
        }
        if s[i] == 'B' {
            has_B = true;
        }
        if s[i] == 'C' {
            has_C = true;
        }
        if has_A && has_B && has_C {
            print!("{}", i + 1);
            break;
        }
    }
    // println!("{}", n);
}
// それだけ抽出するより元の長さと同じやつでvecつくって、連続かどうかを判定するほうが楽。
// vecで連続している数の最大数を求める典型問題の解き方。
fn b() {
    input! {
      n:usize,
      d:usize,
      s:[Chars;n]
    }

    let mut all_hima = Vec::<usize>::new();
    for i in 0..d {
        let mut is_all_hima_day = true;
        for j in 0..n {
            if s[j][i] == 'x' {
                is_all_hima_day = false;
            }
        }
        if is_all_hima_day {
            all_hima.push(i);
        }
    }
    dbg!(&all_hima);
    if all_hima.len() == 0 {
        print!("0");
    } else if all_hima.len() == 1 {
        print!("1");
    } else {
        let mut max_hima_continuous = 1;
        let mut now_hima_continuous = 1;
        let mut before_hima = all_hima[0];
        for i in 1..all_hima.len() {
            if all_hima[i] - all_hima[i - 1] == 1 {
                now_hima_continuous += 1;
                max_hima_continuous = std::cmp::max(max_hima_continuous, now_hima_continuous);
            } else {
                now_hima_continuous = 1;
            }
        }
        print!("{}", max_hima_continuous);
    }
}

// dbg マクロはreleaseで出力はしなくなるけど実行時間はすごく遅くなる。
// 便利ではあるがちゃんとコメントアウトしないとTLEの可能性かなりあがる。
// 無駄な探索しないように前処理しておくことがTLE対策で大事
// functional gralh:ノードからの出る本数が１だけの有向グラフ。頻出。無向ならなもりグラフ
//　これはどこからスタートしても絶対cycleにたどり着くから履歴だけ残して移動してできたcycleを使えば簡単。
// バケットソートの要領で通った場所の回数の履歴を残して二回目だったら閉路。
fn c() {
    input! {
      n:usize,
      a:[usize;n]
    }

    let mut g = vec![vec![]; n];

    for i in 0..n {
        g[i].push(a[i] - 1);
    }

    let mut queue_route = Vec::new();
    let mut is_cycle = false;

    for i in 0..n {
        let start_pos = i;
        let mut visited = vec![false; n];
        for j in 0..i {
            // 毎回初期化してると時間かかる。一回探索して少なくともそのスタート地点は通らない。
            visited[j] = true;
        }
        dfs(
            start_pos,
            &g,
            &mut visited,
            start_pos,
            &mut is_cycle,
            &mut queue_route,
        );
        if is_cycle {
            break;
        }
    }
    println!("{}", queue_route.len());
    for i in 0..queue_route.len() {
        print!("{} ", queue_route[i] + 1);
    }
}

fn dfs(
    node: usize,
    g: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    start_node: usize,
    is_cycle: &mut bool,
    queue_route: &mut Vec<usize>,
) {
    visited[node] = true;
    queue_route.push(node);
    // dbg!(&queue_route);
    for next_node in &g[node] {
        if *next_node == start_node {
            *is_cycle = true;
            return;
        }
        if visited[*next_node] {
            continue;
        } else {
            dfs(*next_node, g, visited, start_node, is_cycle, queue_route);
            if *is_cycle {
                // 戻ってきたときにcycleだと判明してたらqueueを触らずにreturn
                return;
            }
        }
    }
    queue_route.pop(); // ひとつ前のノードに戻るときに今いるノードをpopしておく
}
