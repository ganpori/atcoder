// ここを参考にとく
// https://shiganaise.com/rust-abs/
// https://atcoder.jp/contests/abs
// https://qiita.com/drken/items/dc53c683d6de8aeacf5a
// https://atcoder.jp/contests/dp

// rustのコレクションまとめ
// https://qiita.com/garkimasera/items/a6df4d1cd99bc5010a5e

// mod abc;
// mod tessoku;
// https://atcoder.jp/contests/tessoku-book

use proconio::input;
// use proconio::marker::Chars;

fn main() {
    a();
}

fn a() {
    input! {
      n:usize,
    }

    for i in 0..n {}

    println!("{}", n);
}

// fn a() {
//     input! {
//       n:usize,
//     }

//     for i in 0..n {}

//     println!("{}", n);
// }

// ファイルから文字を読み込む
// let string = std::fs::read_to_string("random_01.txt").unwrap();
// let mut s = vec![];
// for c in string.chars() {
//     s.push(c);
