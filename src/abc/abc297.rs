fn a() {
    input! {
      n:usize,
      d:usize,
      t:[usize;n]
    }

    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            println!("{}", t[i]);
            return;
        }
    }

    println!("-1",);
}

fn b() {
    input! {
      s:Chars
    }
    let mut b_pos = vec![];
    let mut count_r = 0;
    let mut k_bool = false;
    for i in 0..8 {
        if s[i] == 'R' {
            count_r += 1;
        }
        if s[i] == 'B' {
            b_pos.push(i);
        }
        if s[i] == 'K' && count_r == 1 {
            k_bool = true;
        }
    }
    let b_bool: bool;
    if (b_pos[0] + b_pos[1]) % 2 == 1 {
        b_bool = true;
    } else {
        b_bool = false;
    }

    if k_bool && b_bool {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn c() {
    input! {
      h:usize,
      w:usize,
      mut s:[Chars;h]
    }

    for i in 0..h {
        for j in 1..w {
            if s[i][j] == 'T' && s[i][j - 1] == 'T' {
                s[i][j - 1] = 'P';
                s[i][j] = 'C';
            }
        }
        for j in 0..w {
            print!("{}", s[i][j]);
        }
        println!();
    }
}

fn d() {
    input! {
     mut a:usize,
     mut b:usize,
    }

    let mut count = 0;
    while a != 0 && b != 0 {
        if a > b {
            count += a / b;
            a = a % b;
        } else {
            count += b / a;
            b = b % a;
        }
    }
    println!("{}", count - 1);
}
