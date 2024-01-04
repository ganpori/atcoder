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

fn c() {
    input! {
      n:usize,
      m:usize,
      a:[usize;n],
      b:[usize;m]
    }

    let mut ab = [a.clone(), b.clone()].concat();
    // dbg!(&ab);
    ab.sort();
    let set_a: HashSet<usize> = a.into_iter().collect();
    let set_b: HashSet<usize> = b.into_iter().collect();

    let mut aa = vec![];
    let mut bb = vec![];
    for i in 0..n + m {
        if set_a.contains(&ab[i]) {
            aa.push(i + 1);
        } else {
            bb.push(i + 1);
        }
    }

    for i in 0..n {
        print!("{} ", aa[i]);
    }
    println!("");
    for i in 0..m {
        print!("{} ", bb[i]);
    }
}
