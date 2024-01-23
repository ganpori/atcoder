use itertools::Itertools;
#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      n:usize,
      m:usize,
      s:[Chars;n]
    }

    let mut count = 0;
    for comb in s.iter().combinations(2) {
        let s1 = comb[0];
        let s2 = comb[1];
        let mut can_solve = true;
        for i in 0..m {
            if s1[i] == 'x' && s2[i] == 'x' {
                can_solve = false;
            }
        }
        if can_solve {
            count += 1;
        }
    }
    println!("{}", count);
}
