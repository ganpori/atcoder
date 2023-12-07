fn a() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:[usize;n]
    }
    for i in 0..n {
        if a + b == c[i] {
            println!("{}", i + 1);
        }
    }
}
