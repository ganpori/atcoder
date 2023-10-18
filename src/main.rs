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
        if (len_s as i32 - len_t as i32).abs() > 1 {
            continue;
        }
        if len_s == len_t {
            for i in 0..len_t {
                if &s[..i] == &t[..i] && &s[i + 1..] == &t[i + 1..] {
                    list_true.push(j + 1);
                    break;
                }
            }
        } else if len_s < len_t {
            for i in 0..len_t {
                if &s[..i] == &t[..i] && &s[i..] == &t[i + 1..] {
                    // sは&String型
                    // ポインタ同士だけど値比較できてるんか？？？
                    list_true.push(j + 1);
                    break;
                }
            }
        } else if len_s > len_t {
            for i in 0..len_s {
                if &s[..i] == &t[..i] && &s[i + 1..] == &t[i..] {
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
