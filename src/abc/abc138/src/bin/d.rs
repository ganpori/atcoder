use std::collections::VecDeque;

#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      q:usize,
      ab:[[usize;2];n-1],
      px:[[usize;2];q]
    }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[ab[i][0] - 1].push(ab[i][1] - 1);
        g[ab[i][1] - 1].push(ab[i][0] - 1);
    }

    let mut count_vec = vec![0; n];
    for i in 0..q {
        count_vec[px[i][0] - 1] = px[i][1] + count_vec[px[i][0] - 1];
    }

    let mut sum_count_vec = vec![0; n];

    let start_pos: usize = 0;
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    visited[0] = true;
    queue.push_back(start_pos);
    sum_count_vec[0] = count_vec[0];
    while queue.len() > 0 {
        let pos: usize = queue.pop_front().unwrap();
        for next_pos in &g[pos] {
            if !visited[*next_pos] {
                visited[*next_pos] = true;
                queue.push_back(*next_pos);

                sum_count_vec[*next_pos] = sum_count_vec[pos] + count_vec[*next_pos];
            }
        }
    }

    for i in 0..n {
        print!("{} ", sum_count_vec[i]);
    }
}
