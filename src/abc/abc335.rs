fn a() {
    input! {
      s:Chars,
    }
    for i in 0..s.len() {
        if i != s.len() - 1 {
            print!("{}", s[i]);
        }
    }
    print!("4");
}

fn b() {
    input! {
      n:usize,
    }

    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k <= n {
                    println!("{} {} {}", i, j, k);
                }
            }
        }
    }
}

fn c() {
    input! {
      n:usize,
      q:usize
    }
    let mut trajectry_head: Vec<[isize; 2]> = vec![];
    for i in 0..n {
        trajectry_head.push([n as isize - i as isize, 0]);
    }

    for _ in 0..q {
        input! {
          type_q:usize,
        }
        // let mut count_1 = 0;
        if type_q == 1 {
            // count_1 += 1;
            input! {cc:char}
            if cc == 'U' {
                trajectry_head.push([
                    trajectry_head.last().unwrap()[0],
                    trajectry_head.last().unwrap()[1] + 1,
                ]);
            } else if cc == 'D' {
                trajectry_head.push([
                    trajectry_head.last().unwrap()[0],
                    trajectry_head.last().unwrap()[1] - 1,
                ]);
            } else if cc == 'L' {
                trajectry_head.push([
                    trajectry_head.last().unwrap()[0] - 1,
                    trajectry_head.last().unwrap()[1],
                ]);
            } else if cc == 'R' {
                trajectry_head.push([
                    trajectry_head.last().unwrap()[0] + 1,
                    trajectry_head.last().unwrap()[1],
                ]);
            }
        } else if type_q == 2 {
            input! {p:usize}
            let p_index = trajectry_head.len() - p;
            // dbg!(&trajectry_head, p_index);
            println!(
                "{} {}",
                trajectry_head[p_index][0], trajectry_head[p_index][1]
            );
        }
    }
}

fn d() {
    input! {
      n:usize,
    }

    let mut grid: Vec<Vec<isize>> = vec![vec![0; n]; n];
    grid[(n - 1) / 2][(n - 1) / 2] = (n * n) as isize + 1;
    let list_diff: [[isize; 2]; 4] = [[1, 0], [0, -1], [-1, 0], [0, 1]];
    let mut count = 2;
    let mut pos = [0, 0];

    grid[0][0] = 1;
    while is_contain_zeros(&grid) {
        for diff in list_diff {
            let mut not_turn = true;
            while not_turn {
                let next_pos = [pos[0] + diff[0], pos[1] + diff[1]];
                // dbg!(&next_pos);
                if 0 <= next_pos[0]
                    && next_pos[0] <= n as isize - 1
                    && 0 <= next_pos[1]
                    && next_pos[1] <= n as isize - 1
                {
                    if grid[next_pos[1] as usize][next_pos[0] as usize] == 0 {
                        grid[next_pos[1] as usize][next_pos[0] as usize] = count;
                        count += 1;
                        pos = next_pos;
                    } else {
                        not_turn = false;
                    }
                } else {
                    not_turn = false;
                }
            }
        }
    }
    // dbg!(&grid);
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] != (n * n) as isize + 1 {
                print!("{} ", grid[i][j]);
            } else {
                print!("T ");
            }
        }
        println!();
    }
}

fn is_contain_zeros(matrix: &Vec<Vec<isize>>) -> bool {
    for row in matrix {
        for &element in row {
            if element == 0 {
                return true; //
            }
        }
    }
    false // ここまで到達したら全要素が0なのでtrueを返す
}
