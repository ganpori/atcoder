fn a() {
    input! {
      n:usize,
      w:[String;n]
    }

    let ww: [&str; 5] = ["and", "not", "that", "the", "you"];

    for i in 0..n {
        for www in ww {
            if www == w[i] {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn b() {
    input! {
      r:usize,
      c:usize,
     mut b:[Chars;r]
    }

    for i in 0..r {
        for j in 0..c {
            if b[i][j].is_ascii_digit() {
                let value: isize = b[i][j].to_digit(10).unwrap() as isize;
                b[i][j] = '.';
                let diff: Vec<isize> = (-1 * value..=value).collect();
                // dbg!(&diff);
                for comb in iproduct!(&diff, &diff) {
                    if comb.0.abs() + comb.1.abs() > value {
                        continue;
                    }
                    let h = comb.0 + i as isize;
                    let w = comb.1 + j as isize;
                    // dbg!(&comb, h, w);
                    if 0 <= h && h < r as isize && 0 <= w && w < c as isize {
                        if !b[h as usize][w as usize].is_ascii_digit() {
                            b[h as usize][w as usize] = '.';
                        }
                    }
                }
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
