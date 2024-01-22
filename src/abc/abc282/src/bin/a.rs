#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      k:usize,
    }
    let a_num = 'A' as u8;
    for i in 0..k {
        print!("{}", (a_num + i as u8) as char);
    }
}
