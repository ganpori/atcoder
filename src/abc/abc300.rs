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

fn b() {
    input! {
        h:usize,
        w:usize,
        a:[Chars;h],
        b:[Chars;h]

    }

    for hs in 0..=h - 1 {
        for ws in 0..=w - 1 {
            let mut is_same = true;
            for i in 0..h {
                for j in 0..w {
                    if a[(i + hs) % h][(j + ws) % w] != b[i][j] {
                        is_same = false;
                    }
                }
            }
            if is_same {
                print!("Yes");
                return;
            }
        }
    }
    print!("No");
}
