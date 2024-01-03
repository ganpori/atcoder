fn a() {
    input! {
      n:usize,
      s:Chars
    }

    for i in 1..n {
        if s[i] == s[i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn b() {
    input! {
      s:[Chars;8],
    }

    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                println!("{}{}", ('a' as u8 + j as u8) as char, 8 - i);
                return;
            }
        }
    }
}

fn c() {
    input! {
        n:usize,
        x:isize,
        mut a:[isize;n]
    }
    a.sort();
    let mut j: usize = 0;
    for i in 0..n {
        while a[j] - a[i] < x && j < n - 1 {
            j += 1;
        }
        let diff = a[j] - a[i];
        if diff == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
