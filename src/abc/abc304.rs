//  タプルで文字列と数値を同時に受け取ることも可能。
//  sa:[(String,usize);n]
// 変な要素でも比較する方法。下記の場合(String,usize)型のusizeが最小の要素が得られる。
// let min_age = sa.iter().min_by_key(|&x| x.1).unwrap();
fn a() {
    input! {
      n:usize,
      sa:[(String,usize);n]
    }

    let (index_min, sa_min) = sa
        .iter()
        .enumerate()
        .min_by_key(|(i, element)| element.1)
        .unwrap();
    for i in index_min..n {
        println!("{}", sa[i].0);
    }
    for i in 0..index_min {
        println!("{}", sa[i].0);
    }
}
