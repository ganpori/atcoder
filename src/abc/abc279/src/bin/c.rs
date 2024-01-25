use std::collections::HashMap;

#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      h:usize,
      w:usize,
      s:[Chars;h],
      t:[Chars;h]
    }

    let mut s_map: HashMap<Vec<char>, usize> = HashMap::new();
    let mut t_map: HashMap<Vec<char>, usize> = HashMap::new();

    for j in 0..w {
        let mut s_col = vec![];
        let mut t_col = vec![];
        for i in 0..h {
            s_col.push(s[i][j]);
            t_col.push(t[i][j]);
        }
        *s_map.entry(s_col).or_insert(0) += 1;
        *t_map.entry(t_col).or_insert(0) += 1;
    }

    for col in s_map.keys() {
        if !t_map.contains_key(col) {
            println!("No");
            return;
        } else {
            if t_map.get(col).unwrap() != s_map.get(col).unwrap() {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
