fn a() {
    input! {
      s:Chars,
    }

    for i in 0..s.len() {
        if s[i].is_ascii_uppercase() {
            println!("{}", i + 1);
            return;
        }
    }
}

fn b() {
    input! {
      n:usize,
     mut  x:[usize;5*n]
    }
    x.sort();
    let mut sum = 0;

    for i in n..4 * n {
        sum += x[i];
    }
    println!("{}", sum as f64 / (3.0 * n as f64));
}
