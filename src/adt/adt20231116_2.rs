// strから数値への変換はparseよりto_digitの方がわかりよい
fn a() {
    input! {
      x:String,
    }

    let mut integer_str = String::new();
    let mut integer: usize = 0;
    for (i, c) in x.chars().enumerate() {
        if c == '.' {
            let value = x.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
            if value >= 5 {
                println!("{}", integer + 1);
            } else {
                println!("{}", integer);
            }
            return;
        } else {
            integer_str.push(c);
            integer = integer_str.parse().unwrap();
        }
    }
}

fn b() {
    input! {
      card:[usize;5],
    }
    let mut set_card = HashSet::new();
    let num_init = card[0];
    let mut sum = 0;
    for i in 0..5 {
        set_card.insert(card[i]);
        if card[i] == num_init {
            sum += 1;
        }
    }
    if set_card.len() == 2 && (sum == 2 || sum == 3) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn c() {
    input! {
      s1:String,
      s2:String,
      s3:String,
      t:Chars,
    }
    for i in 0..t.len() {
        if t[i] == '1' {
            print!("{}", s1);
        } else if t[i] == '2' {
            print!("{}", s2);
        } else if t[i] == '3' {
            print!("{}", s3);
        }
    }
}

fn d() {
    input! {
      n:usize,
      k:usize,
      a:[usize;k],
      xy:[[isize;2];n]
    }

    let mut r_square_min = vec![usize::MAX; n];
    for i in 0..k {
        let index = a[i] - 1;
        let x = xy[index][0];
        let y = xy[index][1];
        for j in 0..n {
            let r_square: usize = ((x - xy[j][0]).pow(2) + (y - xy[j][1]).pow(2)) as usize;
            r_square_min[j] = std::cmp::min(r_square_min[j], r_square);
        }
    }
    let max_r_square = r_square_min.iter().max().unwrap();
    print!("{}", (*max_r_square as f64).sqrt());
}

fn e() {
    input! {
      n:usize,
      s:Chars
    }

    let mut max_level = 0;
    let mut has_kushi = false;
    let mut dango_continue = 0;
    for i in 0..n {
        if s[i] == '-' {
            has_kushi = true;
            max_level = std::cmp::max(max_level, dango_continue);
            dango_continue = 0;
        } else if s[i] == 'o' {
            dango_continue += 1;
            if has_kushi {
                max_level = std::cmp::max(max_level, dango_continue);
            }
        }
    }
    if max_level == 0 {
        println!("-1");
    } else {
        println!("{}", max_level);
    }
}
