// 固定する場所をaにすると二重ループ、bにすると一重ループ
fn c() {
    input! {
      n:usize,
     mut  a:[usize;n],
     mut  b:[usize;n],
     mut  c:[usize;n]
    }
    a.sort();
    c.sort_by(|a, b| b.cmp(a));

    let mut count = 0;
    for i in 0..n {
        let a_index = a.partition_point(|value| value < &b[i]);
        let c_index = c.partition_point(|value| value > &b[i]);
        count += a[0..a_index].len() * c[0..c_index].len();
    }

    println!("{}", count);
}
