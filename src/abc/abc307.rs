fn a() {
    input! {
      n:usize,
      a:[usize;7*n]
    }

    for i in 0..n {
        let mut sum = 0;
        for j in 0..7 {
            sum += a[j + 7 * i]
        }
        print!("{} ", sum);
    }
}

// 長さ M の文字列T が回文であるとは、
// 任意の1≤i≤M についてT の i 文字目と
// (M+1−i)文字目が一致していることをいいます
fn b() {
    input! {
      n:usize,
      s:[Chars;n]
    }
    for i in 0..n {
        for j in 0..n {
            // j,iの順でくっつけた場合も試さないといけない
            // ループを単純にしてi!=jでやるほうがシンプルっぽそう
            if i != j {
                let mut combined_s = s[i].clone();
                combined_s.extend(&s[j]);
                let mut is_kaibun = true;
                // 中心の前後までチェックすれば十分。中心の文字はなんでもよい。
                for k in 0..=combined_s.len() / 2 {
                    if combined_s[k] != combined_s[combined_s.len() - k - 1] {
                        is_kaibun = false;
                    }
                }
                if is_kaibun {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}

// xがaやbより大きい場合を考慮してなかった。いろんな添え字でミスがあった。
//　解けてない。領域をもっと増やしてもっと遠ざけた場所で切り取る必要があった。でも領域広げると計算間に合わない。
// コーディングしたのはaを固定してbを動かす。そしてxになる領域を探索する。
// 公式解説ではxとなる切り取る領域を先に固定してからaとbを動かしていた。そのほうが探索する領域減らせるみたい。なぜかわからんけど。
fn c() {
    // 解けてない。
    input! {
      ha:usize,
      wa:usize,
      a:[Chars;ha],
      hb:usize,
      wb:usize,
      b:[Chars;hb],
      hx:usize,
      wx:usize,
      x:[Chars;hx]
    }

    // 透明シートをつくる
    let mut c = vec![vec!['.'; 31]; 31];

    // cのど真ん中にaをはる
    // ｃにおけるaの始点は[hb,wb]
    let mut num_a_sharp = 0;
    for hi in hb..hb + ha {
        for wi in wb..wb + wa {
            c[hi][wi] = a[hi - hb][wi - wb];
            if a[hi - hb][wi - wb] == '#' {
                num_a_sharp += 1;
            }
        }
    }

    // bをいろんなところに貼る->xができたか探索
    // bを貼る左上の場所を全探索
    for hi in 0..c.len() - hb {
        for wi in 0..c[0].len() - wb {
            let mut c_copy = c.clone();
            let mut num_overwrite = 0;
            let mut num_b_sharp = 0;
            // c_copyにbを貼り付ける
            for hj in hi..hi + hb {
                for wj in wi..wi + wb {
                    if c_copy[hj][wj] == '.' && b[hj - hi][wj - wi] == '#' {
                        c_copy[hj][wj] = '#';
                    } else if c_copy[hj][wj] == '#' && b[hj - hi][wj - wi] == '#' {
                        // dbg!(&c_copy, &c, hi, hj);
                        num_overwrite += 1;
                    }
                    if b[hj - hi][wj - wi] == '#' {
                        num_b_sharp += 1;
                    }
                }
            }

            // 貼り付けた。次はそのcopyにxは存在するか。
            // dbg!(&c_copy, num_overwrite);
            // xを抽出する左上からループ
            for hk in 0..c.len() - hx {
                for wk in 0..c[0].len() - wx {
                    // 対象領域が一致しているか判定するループ
                    let mut has_x = true;
                    let mut num_sharp = 0;
                    for hl in hk..hk + hx {
                        for wl in wk..wk + wx {
                            // dbg!(c_copy.len(), c_copy[0].len());
                            if c_copy[hl][wl] != x[hl - hk][wl - wk] {
                                has_x = false;
                            }
                            if c_copy[hl][wl] == '#' {
                                num_sharp += 1;
                            }
                        }
                    }
                    if has_x {
                        for i in 0..c.len() {
                            println!("{:?}", c_copy[i]);
                        }
                        dbg!(num_sharp, num_a_sharp, num_b_sharp, num_overwrite);
                        if num_sharp == (num_a_sharp + num_b_sharp - num_overwrite) {
                            // dbg!(num_sharp, num_a_sharp, num_b_sharp, num_overwrite);
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
