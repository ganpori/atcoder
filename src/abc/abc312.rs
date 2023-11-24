fn a() {
    input! {
      s:String,
    }
    if s == "ACE"
        || s == "BDF"
        || s == "CEG"
        || s == "DFA"
        || s == "EGB"
        || s == "GBD"
        || s == "FAC"
    {
        println!("Yes");
    } else {
        println!("No");
    }
    dbg! {s};
}

// dbgマクロはrust 1.69.0からリリースビルドで結果表示されないようになった。ハッピー。
// xyよりhwのインデックス対応がわかりやすいときがある。
//　問題文読んでどこがi,jかちゃんと確認。
fn b() {
    input! {
      n:usize,
      m:usize,
      s:[Chars;n]
    }

    for h in 0..n - 8 {
        for w in 0..m - 8 {
            dbg!(w, h);
            if is_tak(h, w, &s) {
                println!("{} {}", h + 1, w + 1);
            }
        }
    }
}

fn is_tak(h: usize, w: usize, code: &Vec<Vec<char>>) -> bool {
    let mut is_tak = true;
    let pos_sharp: [[usize; 2]; 18] = [
        [0, 0],
        [0, 1],
        [0, 2],
        [1, 0],
        [1, 1],
        [1, 2],
        [2, 0],
        [2, 1],
        [2, 2],
        [6, 6],
        [6, 7],
        [6, 8],
        [7, 6],
        [7, 7],
        [7, 8],
        [8, 6],
        [8, 7],
        [8, 8],
    ];
    for [w_i, h_j] in pos_sharp {
        // dbg!(w + (4 + w_i) as usize, h + (4 + h_j) as usize);
        if '#' != code[h + h_j][w + w_i] {
            is_tak = false;
        }
    }
    let pos_dot: [[usize; 2]; 14] = [
        [0, 3],
        [1, 3],
        [2, 3],
        [3, 3],
        [3, 2],
        [3, 1],
        [3, 0],
        [5, 8],
        [5, 7],
        [5, 6],
        [5, 5],
        [6, 5],
        [7, 5],
        [8, 5],
    ];
    for [w_i, h_j] in pos_dot {
        if '.' != code[h + h_j][w + w_i] {
            is_tak = false;
        }
    }
    is_tak
}

// 境界でどうなってるかの扱いがなんでも難しい。
// 二分探索で境界が怪しいときは辺に特殊例を場合分けせずに、アルゴリズムに任せる。
// 具体的には探索範囲を広めにとって微妙な境界も探索のアルゴリズムで判断するようにする
fn c() {
    input! {
        n:usize,
        m:usize,
    mut a:[usize;n], //　売り手
    mut b:[usize;m]  // 買い手
        }
    a.sort();
    b.sort();
    // 人数の差は単調増加関数

    let mut left_x = a[0] - 1; // 売り手
    let mut right_x = b[m - 1] + 1; // 買い手

    if left_x > right_x {
        print!("{}", right_x);
        return;
    }
    let mut middle_x = (left_x + right_x) / 2;
    while left_x < right_x {
        let mut num_urite = 0;
        for i in 0..n {
            if a[i] <= middle_x {
                num_urite += 1;
            } else {
                break;
            }
        }
        let mut num_kaite = 0;
        for i in 0..m {
            if b[m - i - 1] >= middle_x {
                num_kaite += 1;
            } else {
                break;
            }
        }
        let num_diff = num_urite - num_kaite;
        dbg!(middle_x, left_x, right_x, num_diff);
        if num_diff > 0 {
            right_x = middle_x
        } else if num_diff < 0 {
            left_x = middle_x + 1;
        } else if num_diff == 0 {
            right_x = middle_x
        }
        middle_x = (left_x + right_x) / 2;
    }

    println!("{}", middle_x);
}
