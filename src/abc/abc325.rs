use proconio::input;

pub fn c() {
    use proconio::marker::Chars;
    input! {
        h:usize,
        w:usize,
        list_s:[Chars;h]
    }
    let mut visited = vec![vec![false; w]; h];
    let mut pos: [i32; 2] = [0; 2];
    let mut num_sensor = 0;

    for i in 0..h {
        for j in 0..w {
            pos = [i as i32, j as i32];
            if !visited[pos[0] as usize][pos[1] as usize]  // 未訪問
                && list_s[pos[0] as usize][pos[1] as usize] == '#'
            //　かつセンサー
            {
                dfs(pos, &mut visited, &list_s);
                num_sensor += 1;
            }
        }
    }
    println!("{}", num_sensor);
}

fn dfs(pos: [i32; 2], visited: &mut Vec<Vec<bool>>, list_s: &Vec<Vec<char>>) {
    visited[pos[0] as usize][pos[1] as usize] = true;

    let h = list_s.len() as i32;
    let w = list_s[0].len() as i32;

    let dh = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dw = [1, 1, 1, 0, 0, -1, -1, -1];
    for i in 0..dh.len() {
        let next_h = pos[0] + dh[i];
        let next_w = pos[1] + dw[i];
        if 0 <= next_h && next_h < h && 0 <= next_w && next_w < w {
            if visited[next_h as usize][next_w as usize] == false
                && list_s[next_h as usize][next_w as usize] == '#'
            {
                dfs([next_h, next_w], visited, list_s);
            }
        }
    }
}
