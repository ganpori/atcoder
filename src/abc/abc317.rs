fn a() {
    input!(
        n:usize,
        h:usize,
        x:usize,
        p:[usize;n]
    );

    for i in 0..n {
        if p[i] + h >= x {
            println!("{}", i + 1);
            return;
        }
    }
}

fn b() {
    input!(
        n:usize,
    mut a:[usize;n]
    );
    a.sort();
    for i in 1..n {
        if a[i] - a[i - 1] == 2 {
            println!("{}", a[i] - 1);
        }
    }
}

// dfsでいった場所の履歴を消してくことで全パターンでの経路を探索できる。
// その時の状態を記憶しながら探索すれば全経路での状態を確認できる。
// ただ普通のdfsとくらべめちゃくちゃ経路が増えるので演算時間増えるし、状態を確認する計算コストもめちゃ大きい。
fn c() {
    input!(
        n:usize,
        m:usize,
        abc:[[usize;3];m]
    );

    let mut adjacent_list = vec![vec![]; n];
    for i in 0..m {
        adjacent_list[abc[i][0] - 1].push([abc[i][1] - 1, abc[i][2]]);
        adjacent_list[abc[i][1] - 1].push([abc[i][0] - 1, abc[i][2]]);
    }

    let mut max_length = 0;
    for start_pos in 0..n {
        let mut visited = vec![false; n];
        let mut integrated_cost_from_start_pos: Vec<usize> = vec![0; n];
        dfs(
            start_pos,
            &mut visited,
            &adjacent_list,
            &mut integrated_cost_from_start_pos,
            &mut max_length,
        );
    }
    println!("{}", max_length);
}

fn dfs(
    pos: usize,
    visited: &mut Vec<bool>,
    adjacent_list: &Vec<Vec<[usize; 2]>>,
    integrated_cost_from_pos: &mut Vec<usize>,
    max_length: &mut usize,
) {
    visited[pos] = true;

    for [next_node, next_cost] in adjacent_list[pos].iter() {
        if visited[*next_node] {
            continue;
        } else {
            integrated_cost_from_pos[*next_node] = integrated_cost_from_pos[pos] + next_cost;
            dfs(
                *next_node,
                visited,
                adjacent_list,
                integrated_cost_from_pos,
                max_length,
            );
        }
    }
    // dfsを抜けてひとつ前のノードに戻るときにそのノードには行かなかったことにする。
    // start_posにまで戻った時は全部いってなかったことになる。
    //　でもforループでエッジのイテレーションしてるからもう一度同じところは通るが無限に通るわけではない。
    // 結果全部の行く方法を試すことができる
    visited[pos] = false;
    // 最大値のみを記憶していく
    // 計算コストは結構大きいが入力の制約があるのでセーフ
    for integrated_cost in integrated_cost_from_pos {
        if *max_length <= *integrated_cost {
            *max_length = *integrated_cost;
        }
    }
}
