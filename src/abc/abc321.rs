use proconio::input;
use proconio::marker::Chars;

// charはto_digit(10)で十進の数字に変換可能。unwrapは必要。unwrapはうまくいかない場合例外処理が必要なものを強引に進める下品なやり方。
// &vec[1..]で一要素目を省いたイテレータ作れる
pub fn a() {
    input! {
      n_str_vec:Chars
    }
    let mut left_value = n_str_vec[0].to_digit(10).unwrap();
    for c in &n_str_vec[1..] {
        let right_value = c.to_digit(10).unwrap();
        if right_value >= left_value {
            println!("No");
            return;
        }
        left_value = right_value;
    }
    println!("Yes");
}

pub fn b() {
    input! {
      n:usize,
      x:usize,
      vec_a:[usize;n-1]
    }
    let max_a = vec_a.iter().max().unwrap();
    let min_a = vec_a.iter().min().unwrap();
    let sum_a = vec_a.iter().sum::<usize>();
    if sum_a - min_a < x {
        // 最高得点超えられない
        println!("-1");
    } else if x <= sum_a - max_a {
        //最低得点でも超えちゃう
        println!("0");
    } else {
        let value = x - (sum_a - max_a - min_a);
        println!("{}", value);
    }
}
