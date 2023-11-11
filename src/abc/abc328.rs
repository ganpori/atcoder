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
