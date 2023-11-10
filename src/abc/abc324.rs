use proconio::input;

pub fn c() {
    use proconio::marker::Chars;
    input! {
        n:usize,
        t:Chars,
        list_s:[Chars;n]
    }

    let mut count: u32 = 0;
    let mut out_str = String::new();

    let len_t = t.len();
    for (j, s) in list_s.iter().enumerate() {
        let len_s = s.len();
        if (len_s as i32 - len_t as i32).abs() > 1 {
            continue;
        }
        let min_len = std::cmp::min(len_s, len_t);

        let mut num_same_front = 0;
        for i in 0..min_len {
            if s[i] == t[i] {
                // println!("s[i]:{}", s[i]);
                // println!("t[i]:{}", t[i]);
                num_same_front += 1;
            } else {
                break;
            }
        }
        let mut num_same_back = 0;
        for i in 0..min_len {
            if s[len_s - i - 1] == t[len_t - i - 1] {
                // println!("s[len_s - i - 1]:{}", s[(len_s - i) - 1]);
                // println!("t[len_t - i - 1]:{}", t[(len_t - i) - 1]);
                num_same_back += 1;
            } else {
                break;
            }
        }
        if num_same_front == num_same_back && num_same_front == len_t && num_same_front == len_s {
            out_str.push_str(&((j + 1).to_string() + " "));
            count += 1;
        } else if num_same_front + num_same_back >= len_s && len_t == len_s + 1 {
            //追加された
            //一致されたものが追加される可能性あるから==じゃなくて>=
            out_str.push_str(&((j + 1).to_string() + " "));
            count += 1;
        } else if num_same_front + num_same_back + 1 >= len_s && len_t == len_s - 1 {
            // 削除された
            // 回文とかだと削除されても一致の総和は元の数字より大きいので>=
            out_str.push_str(&((j + 1).to_string() + " "));
            count += 1;
        } else if num_same_front + num_same_back + 1 == len_s && len_t == len_s {
            //文字変換
            out_str.push_str(&((j + 1).to_string() + " "));
            count += 1;
        }
    }
    println!("{}", count);
    println!("{}", out_str);
}

pub fn d() {
    use proconio::marker::Chars;
    input! {
      n:usize,
      mut s:Chars,
    }
    s.sort();

    let mut count = 0;
    let max_x: usize = (10usize.pow(n as u32) as f64).sqrt() as usize;
    for i in 0..=max_x {
        let num_candidate = i * i;
        let str_candidate = num_candidate.to_string();
        let mut s_candidate = vec!['0'; s.len()];
        for (j, nth_val) in str_candidate.char_indices() {
            if j < s.len() {
                s_candidate[j] = nth_val;
            }
        }
        s_candidate.sort();
        if s == s_candidate {
            count += 1;
        }
    }
    println!("{}", count);
}
