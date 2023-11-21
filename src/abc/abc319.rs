fn a() {
    input! {
        s:String
    }
    if s == "tourist" {
        println!("3858");
    } else if s == "ksun48" {
        println!("3679");
    } else if s == "Benq" {
        println!("3658");
    } else if s == "Um_nik" {
        println!("3648");
    } else if s == "apiad" {
        println!("3638");
    } else if s == "Stonefeang" {
        println!("3630");
    } else if s == "ecnerwala" {
        println!("3613");
    } else if s == "mnbvmar" {
        println!("3555");
    } else if s == "newbiedmy" {
        println!("3516");
    } else if s == "semiexp" {
        println!("3481");
    }
}

fn b() {
    input! {
        n:usize
    }
    let mut c = '-';
    for j in 1..=9 {
        if n % j == 0 {
            c = std::char::from_digit(j as u32, 10).unwrap();
            break;
        }
    }
    print!("{}", c);

    for i in 1..=n {
        let mut c = '-';
        for j in 1..=9 {
            if n % j == 0 && i % (n / j) == 0 {
                c = std::char::from_digit(j as u32, 10).unwrap();
                break;
            }
        }
        print!("{}", c);
    }
}

// 全列挙したらいいじゃんという方針はあってた
// permutationsはvec.iter().permutationsか[0..9].permutations()でitertoolsから呼び出せる
// permの数値が何を表しているか、インデックスが何を表しているかがイメージしづらかった。
// 全列挙したらいいじゃんなのであれば条件も全列挙する必要がある。頑張ればかけた。
fn c() {
    use ::itertools::Itertools;
    input! {
        c:[usize;9]
    }
    // println!("{:?}", c);
    let all_trial: usize = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;
    let array_target: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    let mut count_gakkari: usize = 0;

    for open_order_perm in (0..=8 as usize).permutations(9) {
        let mut is_gakkari = false;
        for condition in array_target {
            if c[condition[0]] == c[condition[1]]
                && open_order_perm[condition[0]] < open_order_perm[condition[2]]
                && open_order_perm[condition[1]] < open_order_perm[condition[2]]
            {
                is_gakkari = true;
            } else if c[condition[0]] == c[condition[2]]
                && open_order_perm[condition[0]] < open_order_perm[condition[1]]
                && open_order_perm[condition[2]] < open_order_perm[condition[1]]
            {
                is_gakkari = true;
            } else if c[condition[1]] == c[condition[2]]
                && open_order_perm[condition[1]] < open_order_perm[condition[0]]
                && open_order_perm[condition[2]] < open_order_perm[condition[0]]
            {
                is_gakkari = true;
            }
        }
        if is_gakkari {
            count_gakkari += 1;
        }
    }
    let count_not_gakkari = all_trial - count_gakkari;
    let probability = count_not_gakkari as f64 / all_trial as f64;
    println!("{}", probability);
}
// 二分探索と気づくのがムズイ
// 単調増加関数だから二分探索で行けるらしい
// 二分探索の境界の扱いはその都度考えないといけないっぽい、ムズイ
// シミュレーションできる
//　下限の設定がおかしかった、シミュレーションで全探索すると時間がかかりすぎる。
fn d() {
    input! {
        n:usize,
        m:usize,
        l:[usize;n]
    }

    let mut num_sum_words = 0;
    for num in &l {
        num_sum_words += num;
    }
    let mut lower_width = *l.iter().max().unwrap(); // 少なくともwidthはこの値より大きい
    let mut upper_width = num_sum_words + n - 1;
    let mut width = (lower_width + upper_width) / 2;
    loop {
        //シミュレーションしてらいんのかずを数える
        let mut sum_line: usize = l[0];
        let mut num_lines: usize = 1;
        for i in 1..n {
            if sum_line + 1 + l[i] <= width {
                //その行に単語追加
                sum_line += 1 + l[i];
            } else {
                // 次の行にいってその単語を追加
                sum_line = l[i];
                num_lines += 1;
            }
        }

        if num_lines > m {
            lower_width = width + 1;
        } else if num_lines < m {
            upper_width = width;
        } else if num_lines == m {
            upper_width = width;
        }
        width = (lower_width + upper_width) / 2;
        if width == lower_width && width == upper_width {
            break;
        }
    }
    println!("{}", width);
}
