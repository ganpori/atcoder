fn a() {
    input! {
      s:[usize;8],
    }

    let mut ok = true;
    for i in 0..8 {
        if s[i] < 100 || 675 < s[i] {
            ok = false;
        }
        if s[i] % 5 != 0 {
            ok = false;
        }
    }
    for i in 1..8 {
        if s[i - 1] > s[i] {
            ok = false;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
    // println!("{}", n);
}

// hashmapのkeyはvecでもできるときはある。
// 条件はその要素がEqとHashのトレイトを実装していること
// hashmapの値を取り出すときはgetを使うとOption(<T>)が帰ってくる。
// Noneがない場合はmap[i]とかで取り出してもよい。
fn b() {
    input! {
      n:usize,
      m:usize,
      c:[Chars;n],
      d:[Chars;m],
      p:[usize;m+1]
    }
    let mut price_map = HashMap::new();
    for i in 0..m {
        price_map.insert(&d[i], p[i + 1]);
    }

    dbg!(&price_map, &c);
    let mut sum = 0;
    for i in 0..n {
        match price_map.get(&c[i]) {
            Some(value) => sum += value,
            None => sum += p[0],
        }
    }

    println!("{}", sum);
}

//　u128でもオーバーフローしちゃう。全部の最小公倍数ではなく隣同士で式変形してsortするようにすれば小さい整数で扱える。
// sortのときstable sortかに注意。rustのsortはtimsortでこれはstable。
// |x| *x+1がとかがラムダ式、クロージャ。あくまで関数である。適当な変数オブジェクトに入れることも可能。
// 大体はその場で使うだけなのでインラインでクロージャを書く。関数なので受け取る引数の数、種類は自由にできる。
//  let mut p: Vec<(usize, &(usize, usize))> = ab.iter().enumerate().collect();でインデックスを持つvecが作れる
fn c() {
    input! {
      n:usize,
      ab:[[usize;2];n]
    }

    let mut order = vec![[0; 3]; n];
    for i in 0..n {
        order[i] = [ab[i][0], ab[i][1], i];
    }
    order.sort_by(|x, y| ((y[0] + y[1]) * x[1]).cmp(&((&x[0] + x[1]) * y[1])));

    for i in 0..n {
        print!("{} ", order[i][2] + 1);
    }
}

// 幅優先探索の時はqueueにpushすると同時にvisitedを更新しないと無駄にqueueにたまっていく
// 無駄にたまるので計算時間とメモリ量爆増する。
fn d() {
    input! {
      h:usize,
      w:usize,
      s:[Chars;h]
    }

    let diff: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let mut visited = vec![vec![false; w]; h];
    let mut queue_next = VecDeque::new();
    let mut pos = (0, 0, s[0][0]);
    visited[0][0] = true;
    queue_next.push_back(pos);
    while queue_next.len() > 0 && !visited[h - 1][w - 1] {
        pos = queue_next.pop_front().unwrap();

        for [h_diff, w_diff] in diff {
            let next_h = pos.0 as i32 + h_diff;
            let next_w = pos.1 as i32 + w_diff;
            // 座標判定、visited判定、snuke判定で行くか決める
            if 0 <= next_h && next_h <= h as i32 - 1 && 0 <= next_w && next_w <= w as i32 - 1 {
                if is_snuke(pos.2, s[next_h as usize][next_w as usize])
                    && !visited[next_h as usize][next_w as usize]
                {
                    queue_next.push_back((
                        next_h as usize,
                        next_w as usize,
                        s[next_h as usize][next_w as usize],
                    ));
                    visited[next_h as usize][next_w as usize] = true;
                }
            }
        }
    }
    if visited[h - 1][w - 1] {
        print!("Yes");
    } else {
        print!("No");
    }
}

fn is_snuke(c: char, c_next: char) -> bool {
    if c == 's' && c_next == 'n' {
        return true;
    } else if c == 'n' && c_next == 'u' {
        return true;
    } else if c == 'u' && c_next == 'k' {
        return true;
    } else if c == 'k' && c_next == 'e' {
        return true;
    } else if c == 'e' && c_next == 's' {
        return true;
    }
    false
}
