use proconio::input;

// ここを参考にとく
// https://shiganaise.com/rust-abs/
// https://atcoder.jp/contests/abs
// https://qiita.com/drken/items/dc53c683d6de8aeacf5a
// https://atcoder.jp/contests/dp

fn main() {
    c()
}

fn c() {
    input! {
        n:i32,
        t:String,
        list_s:[String;n]
    }

    let len_t = t.len();
    let mut list_true: Vec<usize> = Vec::new();
    for (j, s) in list_s.iter().enumerate() {
        let len_s = s.len();
        if len_s == len_t {
            for i in 0..len_t {
                let new_s = format!("{}{}", &s[..i], &s[i + 1..]); // ascii文字列ならbyte数と文字数が一致してるのでこれでよい
                let new_t = format!("{}{}", &t[..i], &t[i + 1..]);
                if new_s == new_t {
                    list_true.push(j + 1);
                    break;
                }
            }
        } else if len_s < len_t {
            for i in 0..len_t {
                let new_t = format!("{}{}", &t[..i], &t[i + 1..]);
                // println!("{}", new_t);
                // println!("{}", &new_t);
                if new_t == *s {
                    // sは&String型
                    // ポインタ同士だけど値比較できてるんか？？？
                    list_true.push(j + 1);
                    break;
                }
            }
        } else if len_s > len_t {
            for i in 0..len_s {
                let new_s = format!("{}{}", &s[..i], &s[i + 1..]);
                if new_s == t {
                    list_true.push(j + 1);
                    break;
                }
            }
        }
    }
    println!("{}", list_true.len());
    let mut out_str = String::new();
    for value in &list_true {
        out_str.push_str(&value.to_string());
        out_str.push_str(" ");
        continue;
    }
    println!("{}", out_str);
}
