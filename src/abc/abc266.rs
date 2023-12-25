fn a() {
    input! {
        s:Chars
    }
    println!("{}", s[(s.len()) / 2]);
}

fn b() {
    input! {
      n:isize,

    }

    for x in 0..998244353 {
        if (n - x) % 998244353 == 0 {
            println!("{}", x);
            return;
        }
    }
}
