fn a() {
    input! {
      n:usize,
      ab:[[isize;2];n]
    }
    for i in 0..n {
        println!("{}", ab[i][0] + ab[i][1]);
    }
}

//sortは各要素が持つ比較手法を使って比較する
// stringは違う文字が出てきた時点でunicordのコードポイントの値で比較する。
// それは辞書順になるといって差し支えない
fn b() {
    input! {
      n:usize,
      k:usize,
      s:[String;n]
    }
    let mut sk: Vec<&String> = s.iter().take(k).collect();
    sk.sort();
    for i in 0..k {
        println!("{}", sk[i]);
    }
}
