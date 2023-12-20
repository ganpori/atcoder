fn a() {
    input! {
        r:usize,
        c:usize,
        a:[[usize;2];2]
    }
    println!("{}", a[r - 1][c - 1]);
}
