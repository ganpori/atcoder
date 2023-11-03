// ここを参考にとく
// https://shiganaise.com/rust-abs/
// https://atcoder.jp/contests/abs
// https://qiita.com/drken/items/dc53c683d6de8aeacf5a
// https://atcoder.jp/contests/dp

mod abc;
mod tessoku;
// https://atcoder.jp/contests/tessoku-book

use proconio::input;

fn main() {
    tessoku::binary_search::a13();
    // a13();
}

fn c() {
    input! {
        n:i32,
        t:String,
        list_s:[String;n]
    }

    let mut count: u32 = 0;
    let mut out_str = String::new();

    let len_t = t.len();
    for (j, s) in list_s.iter().enumerate() {
        let len_s = s.len();
        if (len_s as i32 - len_t as i32).abs() > 1 {
            continue;
        }
        if len_s == len_t {
            for i in 0..len_t {
                if &s[..i] == &t[..i] && &s[i + 1..] == &t[i + 1..] {
                    out_str.push_str(&((j + 1).to_string() + " "));
                    count += 1;
                    break;
                }
            }
        } else if len_s < len_t {
            for i in 0..len_t {
                if &s[..i] == &t[..i] && &s[i..] == &t[i + 1..] {
                    // sは&String型
                    // ポインタ同士だけど値比較できてるんか？？？
                    out_str.push_str(&((j + 1).to_string() + " "));
                    count += 1;
                    break;
                }
            }
        } else if len_s > len_t {
            for i in 0..len_s {
                if &s[..i] == &t[..i] && &s[i + 1..] == &t[i..] {
                    out_str.push_str(&((j + 1).to_string() + " "));
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("{}", count);
    println!("{}", out_str);
}
