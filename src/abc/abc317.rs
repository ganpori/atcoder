fn a() {
    input!(
        n:usize,
        h:usize,
        x:usize,
        p:[usize;n]
    );

    for i in 0..n {
        if p[i] + h >= x {
            println!("{}", i + 1);
            return;
        }
    }
}

fn b() {
    input!(
        n:usize,
    mut a:[usize;n]
    );
    a.sort();
    for i in 1..n {
        if a[i] - a[i - 1] == 2 {
            println!("{}", a[i] - 1);
        }
    }
}
