use proconio::input;

pub fn a() {
    input! {
      n:usize,
      x:usize,
      s:[usize;n]
    }

    let mut sum_s = 0;
    for s_value in s.iter() {
        if *s_value <= x {
            sum_s += s_value;
        }
    }
    println!("{}", sum_s);
}

// 数値から&strにするにはいったんstringを経由する
// charsは&strやstringから要素に対するイテレータを生成する,charを返す？
// chars().count()のcountコンシューマで文字数を数えるstr.len()だとバイト数になる
// chars().collect<Vec<chars>>()でcharのvecを作れるvec_char[0]とかできるようになるから便利
// collect_vec()だともっと簡単にvecにできる
// charsは文字のみのイテレータ.char_indices()はReturns an iterator over the chars of a string slice, and their positions.
pub fn b() {
    input! {
      n:usize,
      vec_days:[usize;n],
    }
    let mut count = 0;
    for month in 1..=n {
        let month_str = &month.to_string();
        let vec_char_month = month_str.chars().collect_vec();
        let char_month = vec_char_month[0];
        let char_month = month_str.chars().nth(0).unwrap();

        for day in 1..=vec_days[month - 1] {
            let day_str = &day.to_string();
            // let vec_day_chars = day_str.chars().collect_vec();

            let all_str = format!("{}{}", month_str, day_str);
            let mut count_same_char = 0;
            for char in all_str.chars() {
                if char == char_month {
                    count_same_char += 1;
                }
            }
            if count_same_char == all_str.chars().count() {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

// rustのvecはそもそもがスタック構造になってる
// stringやvecのまま中央の値を削除したりするとほかの要素のメモリ番地をずらす必要があるためN要素あるならO(N)ほどかかってしまう
// 中央の値をずらすをN回やると演算量はO(N^2)
// popやpushはO(1)で演算できるからそれだけをN解で済むならO(N)ぐらいですませられる
// vecでi番目を参照するとかの計算コストはO(1)ぐらいっぽい
// ただStringでそのまま参照しようとするとstr.chars().nth(i)とかになってcharsでイテレータをN回つくることになる
//　そんな場合はchars().collect_vec()でvecにしてしまってvecで参照すると早い。
pub fn d() {
    input! {
      s:String
    }

    if s.len() <= 2 {
        println!("{}", s);
        return;
    } else {
        let mut vec_s = Vec::<char>::new();
        for s_char in s.chars() {
            vec_s.push(s_char);
            if vec_s.len() <= 2 {
                continue;
            } else {
                if vec_s[vec_s.len() - 3] == 'A'
                    && vec_s[vec_s.len() - 2] == 'B'
                    && vec_s[vec_s.len() - 1] == 'C'
                {
                    vec_s.pop();
                    vec_s.pop();
                    vec_s.pop();
                }
            }
        }
        for char_s in vec_s {
            print!("{}", char_s);
        }
    }
}

// leftとrightは中間の+-1に設定するみたい。それで間に抜けがないかはどうやって判断するんだろう。
// 二分探索の場合分けは<,>,=を分けてやったほうがわかりやすい。
// 二分探索はターゲットの値を与えてそれに対して境界を取得するようにするほうがわかりよい
// 二分探索の無限ループでleft < rigetという条件を入れることで適当なタイミングでループ終わる
pub fn d() {
    input! {
      n:usize,
      m:usize,
      p:usize,
      mut list_a:[usize;n],
      mut list_b:[usize;m],
    }
    list_a.sort();
    list_b.sort();
    let mut cumulative_b: Vec<usize> = vec![0; m];
    cumulative_b[0] = list_b[0];
    for i in 1..m {
        cumulative_b[i] = cumulative_b[i - 1] + list_b[i];
    }

    let mut sum_price = 0;
    for a in &list_a {
        let mut left_index: usize = 0;
        let mut right_index: usize = list_b.len() - 1;
        let mut k_dash = (right_index + left_index) / 2;
        if *a >= p {
            k_dash = 0;
        } else {
            let target_value = p - a;
            while left_index < right_index {
                // target_valueより大きくなる最小のindexを返す
                let candidate_value = list_b[k_dash];
                if candidate_value > target_value {
                    right_index = k_dash;
                } else if candidate_value < target_value {
                    left_index = k_dash + 1;
                } else if candidate_value == target_value {
                    //同じの場合その値の一番左のindexに行きたい。=時の処理で終端の微妙な位置がかわる
                    right_index = k_dash;
                }
                k_dash = (right_index + left_index) / 2;
            }
        }
        if k_dash == m - 1 && a + list_b[k_dash] <= p {
            sum_price += m * a + cumulative_b[m - 1];
        } else if k_dash == 0 {
            sum_price += m * p;
        } else {
            sum_price += k_dash * a + (m - k_dash) * p + cumulative_b[k_dash - 1];
        }
    }
    println!("{}", sum_price);
}
