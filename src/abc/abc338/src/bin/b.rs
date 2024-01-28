use im_rc::HashMap;
#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
      s:Chars,
    }

    let mut s_map = HashMap::new();
    for i in 0..s.len() {
        *s_map.entry(s[i]).or_insert(0) += 1;
    }

    let mut vec_key: Vec<char> = vec![];
    let mut max_num: usize = 0;
    for (key, value) in s_map.iter() {
        if *value > max_num {
            vec_key = vec![*key];
            max_num = *value;
        } else if *value == max_num {
            vec_key.push(*key);
        }
    }
    vec_key.sort();
    println!("{}", vec_key[0]);
}
