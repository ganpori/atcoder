fn a() {
    input! {
        n:usize
    }
    let s="3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

    for (i, si) in s.to_string().chars().enumerate() {
        print!("{}", si);
        if n + 1 == i {
            break;
        }
    }
}

// 実装が少しめんどくさいけど地道に頑張るだけ
fn b() {
    input! {
        n:usize
    }

    let mut vec_c = Vec::new();
    let mut vec_a = vec![];
    for i in 0..n {
        input! {c:usize}
        vec_c.push(c);
        input! {a:[usize;c]}
        vec_a.push(a);
    }
    input! {x:usize}

    let mut vec_tousen = vec![];
    let mut num_kake_min = 38;
    for i in 0..n {
        for value in &vec_a[i] {
            if x == *value {
                vec_tousen.push(i);
                num_kake_min = std::cmp::min(num_kake_min, vec_a[i].len());
                break;
            }
        }
    }

    let mut count = 0;
    for b in &vec_tousen {
        if vec_a[*b].len() == num_kake_min {
            count += 1;
        }
    }
    println!("{}", count);
    for b in &vec_tousen {
        if vec_a[*b].len() == num_kake_min {
            print!("{} ", *b + 1);
        }
    }
}

// shiftはvecでrotate_rightもできる。その計算量はO(N)
// vecdequeでpopとpushはどちらからでもO(1)
// 最終的にpopしていくことが必要な事も考えると圧倒的にvecdequeのほうが良い
fn c() {
    input! {
        n:usize,
        m:usize,
        s:Chars,
        c:[usize;n]
    }

    let mut coler_set: Vec<VecDeque<char>> = vec![VecDeque::new(); m];
    for i in 0..n {
        coler_set[c[i] - 1].push_back(s[i]);
    }

    // println!("{:?}", coler_set);

    for j in 0..m {
        let back_char = coler_set[j].pop_back().unwrap();
        coler_set[j].push_front(back_char);
    }

    for i in 0..n {
        print!("{}", coler_set[c[i] - 1].pop_front().unwrap());
    }
}

// 今回は最後の操作とそのタイミングだけ覚えておけばよかった。
// その他の場合を考慮しようとして全部のデータ扱ってたから遅くなった。
// 最初の見極めが大事？全部のデータが必要な場合もある。
fn d() {
    input! {
        n:usize,
        s:Chars,
        q:usize,
    }

    let mut is_x = vec![[0, 0]; n]; // 一つ目があるかないか、二つ目が何回目のqで交換されたか
    let mut c = vec!['a'; n];
    let mut t2 = vec![];
    let mut t3 = vec![];
    for i in 0..q {
        input! {
            tx_i:[usize;2],
            c_i:char,
        }
        if tx_i[0] == 1 {
            is_x[tx_i[1] - 1] = [1, i]; // 同じ個所書き換えられても最新だけわかればよい
            c[tx_i[1] - 1] = c_i;
        } else if tx_i[0] == 2 {
            t2.push(i);
        } else if tx_i[0] == 3 {
            t3.push(i);
        }
    }
    if t2.len() == 0 && t3.len() == 0 {
        for i in 0..n {
            if is_x[i][0] == 1 {
                print!("{}", c[i]);
            } else {
                print!("{}", s[i]);
            }
        }
    } else if t2.len() == 0 && t3.len() != 0 {
        let index_last_t3 = t3[t3.len() - 1];
        for i in 0..n {
            if is_x[i][0] == 1 && is_x[i][1] > index_last_t3 {
                print!("{}", c[i]);
            } else if is_x[i][0] == 1 && is_x[i][1] < index_last_t3 {
                print!("{}", c[i].to_uppercase());
            } else {
                print!("{}", s[i].to_uppercase());
            }
        }
    } else if t2.len() != 0 && t3.len() == 0 {
        let index_last_t2 = t2[t2.len() - 1];
        for i in 0..n {
            if is_x[i][0] == 1 && is_x[i][1] > index_last_t2 {
                print!("{}", c[i]);
            } else if is_x[i][0] == 1 && is_x[i][1] < index_last_t2 {
                print!("{}", c[i].to_lowercase());
            } else {
                print!("{}", s[i].to_lowercase());
            }
        }
    } else {
        // t2,t3どちらも0じゃない
        let index_last_t2 = t2[t2.len() - 1];
        let index_last_t3 = t3[t3.len() - 1];
        if index_last_t2 > index_last_t3 {
            for i in 0..n {
                if is_x[i][0] == 1 && is_x[i][1] > index_last_t2 {
                    print!("{}", c[i]);
                } else if is_x[i][0] == 1 && is_x[i][1] < index_last_t2 {
                    print!("{}", c[i].to_lowercase());
                } else {
                    print!("{}", s[i].to_lowercase());
                }
            }
        } else if index_last_t2 < index_last_t3 {
            for i in 0..n {
                if is_x[i][0] == 1 && is_x[i][1] > index_last_t3 {
                    print!("{}", c[i]);
                } else if is_x[i][0] == 1 && is_x[i][1] < index_last_t3 {
                    print!("{}", c[i].to_uppercase());
                } else {
                    print!("{}", s[i].to_uppercase());
                }
            }
        }
    }
}
