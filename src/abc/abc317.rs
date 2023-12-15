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

fn d() {
    input! {
      n:usize,
      xyz:[[usize;3];n]
    }

    let mut people_need: Vec<_> = vec![0; n];
    for i in 0..n {
        if xyz[i][0] < xyz[i][1] {
            people_need[i] = (xyz[i][0] + xyz[i][1]) / 2 + 1 - xyz[i][0];
        }
    }

    let z_sum: usize = xyz.iter().map(|x| x[2]).sum();
    let z_need: usize = z_sum / 2 + 1;
    let max_people = 100 * 1000_000_000 + 1;
    let mut dp: Vec<Vec<isize>> = vec![vec![max_people; z_sum + 1]; n + 1];
    dp[0][0] = 0;
    let mut sum_people = max_people;
    for i in 1..n + 1 {
        for j in 0..z_sum + 1 {
            let zi = xyz[i - 1][2];
            if dp[i - 1][j] != max_people {
                dp[i][j] = dp[i - 1][j]
            }
            if j >= zi {
                if dp[i - 1][j - zi] != max_people {
                    dp[i][j] =
                        std::cmp::min(dp[i][j], dp[i - 1][j - zi] + people_need[i - 1] as isize);
                }
            }
            if dp[i][j] != max_people && j >= z_need {
                sum_people = std::cmp::min(sum_people, dp[i][j]);
            }
        }
    }
    // dbg!(&dp, &z_sum);
    print!("{}", sum_people);
}
