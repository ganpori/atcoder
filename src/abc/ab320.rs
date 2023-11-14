pub fn a() {
    input! {
        a:usize,
        b:usize,
    }
    let result = a.pow(b as u32) + b.pow(a as u32);
    println!("{}", result);
}
