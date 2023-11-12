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

// x << y	x を y ビット左シフト
// x >> y	x を y ビット右シフト
// 1 << n は2^nを意味する。n+1桁の0b1000....0になる。型は普通のi32とか。
// 各数字を一つずつつかうか、という問題に置き換えられる。順番は一つだから。
pub fn c() {
    input! {
       k:usize
    }

    let list_str = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let n = list_str.len() as u32;
    let mut list_num_usize = Vec::<usize>::new();
    for bit_expression in 1..1usize.rotate_left(n) {
        let mut list_num = Vec::<&str>::new();
        for i in 0..n {
            //シフトする桁数のループ
            // println!("bit_expression = {}", bit_expression);
            // println!("1 << i = {}", 1 << i);
            // println!("bit_expression & 1 << i = {}", (bit_expression & (1 << i)));
            // i bit分シフトさせてその桁の01をチェックする
            if (bit_expression & (1 << i)) >= 1 {
                list_num.push(list_str[i as usize])
            }
        }
        if list_num.len() >= 1 {
            list_num.reverse();
            // println!("{:?}", list_num);
            let num_str = list_num.concat();
            let number: usize = num_str.parse().unwrap();
            if number != 0 {
                list_num_usize.push(number);
            }
        }
    }
    list_num_usize.sort();
    for (i, value) in list_num_usize.iter().enumerate() {
        if i + 1 == k {
            println!("{}", value);
        }
    }
}
