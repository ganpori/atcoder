fn a() {
    input! {
      n:usize,
      ab:[[isize;2];n]
    }
    for i in 0..n {
        println!("{}", ab[i][0] + ab[i][1]);
    }
}
