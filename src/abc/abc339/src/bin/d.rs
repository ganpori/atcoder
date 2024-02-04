use std::collections::VecDeque;

#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      s:[Chars;n]
    }
    let dx = [-1, 0, 1, 0];
    let dy = [0, 1, 0, -1];

    let mut graph_distance = vec![vec![vec![vec![usize::MAX; n]; n]; n]; n];

    let mut pos: Vec<[isize; 2]> = vec![];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                pos.push([i as isize, j as isize]);
            }
        }
    }
    let mut queue = VecDeque::new();
    graph_distance[pos[0][0] as usize][pos[0][1] as usize][pos[1][0] as usize]
        [pos[1][1] as usize] = 0;
    queue.push_front([pos[0], pos[1]]);

    while !queue.is_empty() {
        // dbg!(&queue);
        let [pos1, pos2] = queue.pop_front().unwrap();
        for i in 0..4 {
            let mut next_pos1 = [pos1[0] + dx[i], pos1[1] + dy[i]];
            if next_pos1[0] == -1
                || next_pos1[0] == n as isize
                || next_pos1[1] == -1
                || next_pos1[1] == n as isize
                || s[next_pos1[0] as usize][next_pos1[1] as usize] == '#'
            {
                next_pos1 = [pos1[0], pos1[1]];
            }

            let mut next_pos2 = [pos2[0] + dx[i], pos2[1] + dy[i]];
            if next_pos2[0] == -1
                || next_pos2[0] == n as isize
                || next_pos2[1] == -1
                || next_pos2[1] == n as isize
                || s[next_pos2[0] as usize][next_pos2[1] as usize] == '#'
            {
                next_pos2 = [pos2[0], pos2[1]];
            }
            // dbg!(
            //     &next_pos1,
            //     &next_pos2,
            //     pos1,
            //     pos2,
            //     graph_distance[pos1[0] as usize][pos1[0] as usize][pos2[1] as usize]
            //         [pos2[1] as usize]
            // );
            if graph_distance[next_pos1[0] as usize][next_pos1[1] as usize][next_pos2[0] as usize]
                [next_pos2[1] as usize]
                == usize::MAX
            {
                queue.push_back([next_pos1, next_pos2]);
                graph_distance[next_pos1[0] as usize][next_pos1[1] as usize]
                    [next_pos2[0] as usize][next_pos2[1] as usize] = graph_distance
                    [pos1[0] as usize][pos1[1] as usize][pos2[0] as usize][pos2[1] as usize]
                    + 1;
            }
        }
    }
    let mut result = usize::MAX;
    for i in 0..n {
        for j in 0..n {
            result = std::cmp::min(result, graph_distance[i][j][i][j]);
        }
    }
    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
