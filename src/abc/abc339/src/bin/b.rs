#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      h:isize,
      w:isize,
      n:usize
    }
    let mut pos: [isize; 2] = [0, 0];
    let mut grid = vec![vec![false; w as usize]; h as usize];
    let mut is_black_now = grid[pos[0] as usize][pos[1] as usize];
    let mut direction: [isize; 2] = [-1, 0];
    for i in 0..n {
        // dbg!(&pos);
        is_black_now = grid[pos[0] as usize][pos[1] as usize];

        grid[pos[0] as usize][pos[1] as usize] = !grid[pos[0] as usize][pos[1] as usize];
        if !is_black_now {
            // 白
            if direction == [0, 1] {
                direction = [1, 0];
            } else if direction == [0, -1] {
                direction = [-1, 0];
            } else if direction == [1, 0] {
                direction = [0, -1];
            } else if direction == [-1, 0] {
                direction = [0, 1];
            }
        } else {
            // 黒
            if direction == [0, 1] {
                direction = [-1, 0];
            } else if direction == [0, -1] {
                direction = [1, 0];
            } else if direction == [-1, 0] {
                direction = [0, -1];
            } else if direction == [1, 0] {
                direction = [0, 1];
            }
        }
        pos[0] = pos[0] + direction[0];
        pos[1] = pos[1] + direction[1];
        if pos[0] < 0 {
            pos[0] = pos[0] + h;
        }
        if pos[1] < 0 {
            pos[1] = pos[1] + w;
        }
        pos[0] = pos[0] % h;
        pos[1] = pos[1] % w;
    }
    for i in 0..h as usize {
        for j in 0..w as usize {
            if grid[i][j] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
