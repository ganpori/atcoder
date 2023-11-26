use proconio::input;

pub fn a61() {
    input! {
        n:usize,
        m:usize,
        ab:[[usize;2];m]
    }

    let mut adjacent_list = vec![Vec::<usize>::new(); n];
    for edge in ab.iter() {
        adjacent_list[edge[0] - 1].push(edge[1] - 1);
        adjacent_list[edge[1] - 1].push(edge[0] - 1);
    }

    for (i, node) in adjacent_list.iter().enumerate() {
        let mut str_output = format!("{}{}", (i + 1).to_string(), ": {");
        let s = node
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{}: {{{}}}", i + 1, s);
    }
}

pub fn a62() {
    input! {
        n:usize,
        m:usize,
        vec_edge:[[usize;2];m]
    }
    let mut adjacency_list = vec![Vec::<usize>::new(); n];
    for edge in vec_edge.iter() {
        adjacency_list[edge[0] - 1].push(edge[1] - 1);
        adjacency_list[edge[1] - 1].push(edge[0] - 1);
    }

    // println!("{:?}", adjacency_list);
    let mut vec_visited = vec![false; n];
    dfs(0, &mut vec_visited, &adjacency_list);

    if vec_visited.iter().all(|x| *x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(pos: usize, vec_visited: &mut Vec<bool>, adjacency_list: &Vec<Vec<usize>>) {
    vec_visited[pos] = true;
    for adjacency_node in adjacency_list[pos].iter() {
        if vec_visited[*adjacency_node] {
            continue;
        } else {
            dfs(*adjacency_node, vec_visited, adjacency_list) //ここをmut とか&つけなくていいのなぜかわからん
        }
    }
}



// 幅優先探索の時はqueueにpushすると同時にvisitedを更新しないと無駄にqueueにたまっていく
// 無駄にたまるので計算時間とメモリ量爆増する。
pub fn a63() {
    use std::collections::VecDeque;
    input! {
        n:usize,
        m:usize,
        vec_edge:[[usize;2];m]
    }

    let mut adjacency_list = vec![Vec::<usize>::new(); n];
    for edge in vec_edge.iter() {
        adjacency_list[edge[0] - 1].push(edge[1] - 1);
        adjacency_list[edge[1] - 1].push(edge[0] - 1);
    }
    let mut dist = vec![-1; n];
    let mut queue = VecDeque::new();
    let mut pos = 0;
    dist[pos] = 0;
    queue.push_back(0);

    while queue.len() > 0 {
        pos = queue.pop_front().unwrap();
        for to in &adjacency_list[pos] {
            if dist[*to] == -1 {
                queue.push_back(*to);
                dist[*to] = dist[pos] + 1;
            }
        }
    }
    for val in dist.iter() {
        println!("{}", val);
    }
    
// ダイクストラ法で距離付きのグラフを解く
// 幅優先探索と考え方は同じ。
// スタート位置からの距離を暫定距離を使って更新していく。
// 明らかな最小距離のノードから暫定距離を真の距離に確定させていく。
// 明らかな最小距離のノードを効率よく選ぶためにpriority queueを使う。
fn a64() {
    use std::collections::BinaryHeap;

    input! {
        n:usize,
        m:usize,
        vec_edge:[[usize;3];m]
    }

    let max_dist: usize = m * 10_001;
    let mut adjacent_list = vec![vec![]; n];
    for i in 0..m {
        let a2b_array = [vec_edge[i][1] - 1, vec_edge[i][2]];
        adjacent_list[vec_edge[i][0] - 1].push(a2b_array);
        let b2a_array = [vec_edge[i][0] - 1, vec_edge[i][2]];
        adjacent_list[vec_edge[i][1] - 1].push(b2a_array);
    }
    // println!("{:?}", adjacent_list);

    let mut kakutei = vec![false; n];
    let mut current_dist = vec![max_dist; n];

    let mut queue = BinaryHeap::new();
    current_dist[0] = 0;
    queue.push([-1 * current_dist[0] as i32, 0]); //値の大きいものから取り出されるので-1しておく。中身がvecならvecの一番左を参照されるっぽい
    while queue.len() > 0 {
        // 次に確定する頂点posを決める
        let [_, pos] = queue.pop().unwrap();

        if kakutei[pos as usize] == true {
            continue; // posへの辺が何個もあるがすでにposへの最小のdistが定まっている場合
        }

        //posと隣接する頂点のcurrentの値を更新
        kakutei[pos as usize] = true;
        for [next_node, weight] in &adjacent_list[pos as usize] {
            current_dist[*next_node] = std::cmp::min(
                current_dist[pos as usize] + *weight,
                current_dist[*next_node],
            );
            if kakutei[*next_node] == false {
                queue.push([-1 * current_dist[*next_node] as i32, *next_node as i32]);
            }
        }
    }
    for i in 0..n {
        if current_dist[i] != max_dist {
            println!("{}", current_dist[i]);
        } else {
            println!("-1");
        }
    }
}
