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
