fn a() {
    input! {
      n:usize,
      a:[usize;n]

    }

    for i in 0..n {
        if a[i] % 2 == 0 {
            print!("{} ", a[i]);
        }
    }
}

fn b() {
    input! {
      h:usize,
      w:usize,
      a:[[usize;w];h]
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                print!(".");
            } else {
                let c = ('A' as u8 + a[i][j] as u8 - 1) as char;
                print!("{}", c);
            }
        }
        println!("");
    }
}
