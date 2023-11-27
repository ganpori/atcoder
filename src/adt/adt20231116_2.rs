// strから数値への変換はparseよりto_digitの方がわかりよい
fn a() {
    input! {
      x:String,
    }

    let mut integer_str = String::new();
    let mut integer: usize = 0;
    for (i, c) in x.chars().enumerate() {
        if c == '.' {
            let value = x.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
            if value >= 5 {
                println!("{}", integer + 1);
            } else {
                println!("{}", integer);
            }
            return;
        } else {
            integer_str.push(c);
            integer = integer_str.parse().unwrap();
        }
    }
}

fn b() {
    input! {
      card:[usize;5],
    }
    let mut set_card = HashSet::new();
    let num_init = card[0];
    let mut sum = 0;
    for i in 0..5 {
        set_card.insert(card[i]);
        if card[i] == num_init {
            sum += 1;
        }
    }
    if set_card.len() == 2 && (sum == 2 || sum == 3) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn c() {
    input! {
      s1:String,
      s2:String,
      s3:String,
      t:Chars,
    }
    for i in 0..t.len() {
        if t[i] == '1' {
            print!("{}", s1);
        } else if t[i] == '2' {
            print!("{}", s2);
        } else if t[i] == '3' {
            print!("{}", s3);
        }
    }
}

fn d() {
    input! {
      n:usize,
      k:usize,
      a:[usize;k],
      xy:[[isize;2];n]
    }

    let mut r_square_min = vec![usize::MAX; n];
    for i in 0..k {
        let index = a[i] - 1;
        let x = xy[index][0];
        let y = xy[index][1];
        for j in 0..n {
            let r_square: usize = ((x - xy[j][0]).pow(2) + (y - xy[j][1]).pow(2)) as usize;
            r_square_min[j] = std::cmp::min(r_square_min[j], r_square);
        }
    }
    let max_r_square = r_square_min.iter().max().unwrap();
    print!("{}", (*max_r_square as f64).sqrt());
}

fn e() {
    input! {
      n:usize,
      s:Chars
    }

    let mut max_level = 0;
    let mut has_kushi = false;
    let mut dango_continue = 0;
    for i in 0..n {
        if s[i] == '-' {
            has_kushi = true;
            max_level = std::cmp::max(max_level, dango_continue);
            dango_continue = 0;
        } else if s[i] == 'o' {
            dango_continue += 1;
            if has_kushi {
                max_level = std::cmp::max(max_level, dango_continue);
            }
        }
    }
    if max_level == 0 {
        println!("-1");
    } else {
        println!("{}", max_level);
    }
}

//　対角との内積で判定できるのでは？という考察が間違い
//  二次元平面の角度でも、三次元目を0として扱うことで外積を考えることが可能
// 外積の演算をしてできたベクトルがz軸でどっちの向きかでなす角が180以上か判定可能。
// ただし角度は反時計回りにはかるものとする。
// a_vecとb_vecがあったらax*by-ay*bx>0なら180未満
fn f() {
    input! {
      abcd:[[isize;2];4]
    }

    // ab,ad,bc,ba,cd,cb,da,dc
    let mut vec_edge = [
        [abcd[1][0] - abcd[0][0], abcd[1][1] - abcd[0][1]], // ab
        [abcd[3][0] - abcd[0][0], abcd[3][1] - abcd[0][1]], // ad
        [abcd[2][0] - abcd[1][0], abcd[2][1] - abcd[1][1]], // bc
        [abcd[0][0] - abcd[1][0], abcd[0][1] - abcd[1][1]], // ba
        [abcd[3][0] - abcd[2][0], abcd[3][1] - abcd[2][1]], // cd
        [abcd[1][0] - abcd[2][0], abcd[1][1] - abcd[2][1]], // cb
        [abcd[0][0] - abcd[3][0], abcd[0][1] - abcd[3][1]], //da
        [abcd[2][0] - abcd[3][0], abcd[2][1] - abcd[3][1]], // dc
    ];

    let mut outer_product_z = [0; 4];

    outer_product_z[0] = vec_edge[0][0] * vec_edge[1][1] - vec_edge[0][1] * vec_edge[1][0];
    outer_product_z[1] = vec_edge[2][0] * vec_edge[3][1] - vec_edge[2][1] * vec_edge[3][0];
    outer_product_z[2] = vec_edge[4][0] * vec_edge[5][1] - vec_edge[4][1] * vec_edge[5][0];
    outer_product_z[3] = vec_edge[6][0] * vec_edge[7][1] - vec_edge[6][1] * vec_edge[7][0];

    for i in 0..4 {
        if outer_product_z[i] < 0 {
            print!("No");
            return;
        }
    }
    print!("Yes");
}

// 組み合わせて余りがちょうどや～以下の時にdpが使いやすい。
//　何もわかってないが特に気にしない。
// 三次元の動的計画法らしい。わからんから模写する。
//dp_ijkからj個aが選ばれて総和をDで割った余りがkであるようなときの総和の最大値。
// &usizeはiteratorトレイトを実装していない。
// sumはiteratorトレイトを実装した方に対して提供されている。
// mapを使って要素をデリファレンスする必要がある
fn g() {
    input! {
        n:usize,
        K:usize,
        d:usize,
        a:[usize;n]
    }

    let mut dp: Vec<Vec<Vec<isize>>> = vec![vec![vec![-1; d]; K + 1]; n + 1];
    dp[0][0][0] = 0;

    for i in 0..n {
        for j in 0..=K {
            // =Kであることに注意
            for k in 0..d {
                if dp[i][j][k] == -1 {
                    continue;
                    //　なんで無視するの？
                }
                // a_iを選ばない場合の遷移
                dp[i + 1][j][k] = std::cmp::max(dp[i + 1][j][k], dp[i][j][k]);

                // a_iを選ぶ場合の遷移
                if j != K {
                    dp[i + 1][j + 1][(k + a[i]) % d] = std::cmp::max(
                        dp[i + 1][j + 1][(k + a[i]) % d],
                        dp[i][j][k] + a[i] as isize,
                    );
                }
            }
        }
    }
    println!("{}", dp[n][K][0]);
}
