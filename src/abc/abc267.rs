fn a() {
    input! {
      s:Chars,
    }

    if s[0] == 'M' {
        println!("5");
    } else if s[0] == 'T' && s[1] == 'u' {
        println!("4");
    } else if s[0] == 'W' {
        println!("3");
    } else if s[0] == 'T' && s[1] == 'h' {
        println!("2");
    } else if s[0] == 'F' {
        println!("1");
    }
}

// 情報を列の変数に集約してから判定
fn b() {
    input! {
      s:Chars,
    }
    if s[0] == '1' {
        println!("No");
    } else {
        if s[3] == '0' && s[6] == '1' && (s[7] == '1' || s[1] == '1') {
            println!("Yes");
        } else if s[3] == '1' && s[4] == '1' && s[7] == '0' && s[1] == '0' {
            println!("Yes");
        } else if (s[7] == '1' || s[1] == '1') && (s[2] == '1' || s[8] == '1') && s[4] == '0' {
            println!("Yes");
        } else if s[4] == '1' && s[5] == '1' && s[2] == '0' && s[8] == '0' {
            println!("Yes");
        } else if (s[8] == '1' || s[2] == '1') && s[9] == '1' && s[5] == '0' {
            println!("Yes");
        } else if s[6] == '1' && s[4] == '1' && (s[3] == '0' || s[7] == '0' && s[1] == '0') {
            println!("Yes");
        } else if s[6] == '1'
            && (s[8] == '1' || s[2] == '1')
            && (s[4] == '0' || s[3] == '0' || s[7] == '0' && s[1] == '0')
        {
            println!("Yes");
        } else if s[6] == '1'
            && s[5] == '1'
            && (s[8] == '0' && s[2] == '0'
                || s[4] == '0'
                || s[3] == '0'
                || s[7] == '0' && s[1] == '0')
        {
            println!("Yes");
        } else if s[6] == '1'
            && s[9] == '1'
            && (s[8] == '0' && s[2] == '0'
                || s[4] == '0'
                || s[3] == '0'
                || s[7] == '0' && s[1] == '0'
                || s[5] == '0')
        {
            println!("Yes");
        } else if s[3] == '1'
            && (s[8] == '1' || s[2] == '1')
            && (s[4] == '0' || s[7] == '0' && s[1] == '0')
        {
            println!("Yes");
        } else if s[3] == '1'
            && s[5] == '1'
            && (s[4] == '0' || s[7] == '0' && s[1] == '0' || s[8] == '0' && s[2] == '0')
        {
            println!("Yes");
        } else if s[3] == '1'
            && s[9] == '1'
            && (s[4] == '0'
                || s[7] == '0' && s[1] == '0'
                || s[8] == '0' && s[2] == '0'
                || s[5] == '0')
        {
            println!("Yes");
        } else if (s[7] == '1' && s[1] == '1')
            && s[5] == '1'
            && (s[4] == '0' || s[8] == '0' && s[2] == '0')
        {
            println!("Yes");
        } else if (s[7] == '1' && s[1] == '1')
            && s[9] == '1'
            && (s[4] == '0' || s[8] == '0' && s[2] == '0' || s[5] == '0')
        {
            println!("Yes");
        } else if s[4] == '1' && s[9] == '1' && (s[8] == '0' && s[2] == '0' || s[5] == '0') {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn c() {
    input! {
      n:usize,
      m:usize,
      a:[isize;n]
    }
    let mut vec_m_sum = vec![];

    let initial_m_sum: isize = a.iter().take(m).sum();
    vec_m_sum.push(initial_m_sum);

    for i in 1..n - m + 1 {
        let remove_val = a[i - 1];
        let add_val = a[i + m - 1];
        vec_m_sum.push(vec_m_sum[i - 1] + add_val - remove_val);
    }
    // dbg!(&vec_m_sum);

    let mut initial_b = 0;
    for i in 0..m {
        initial_b += (i as isize + 1) * a[i];
    }
    let mut vec_b: Vec<isize> = vec![initial_b];
    for i in 1..n - m + 1 {
        vec_b.push(vec_b[i - 1] + m as isize * a[i + m - 1] - vec_m_sum[i - 1])
    }

    // dbg!(&vec_b, initial_b);
    println!("{}", vec_b.iter().max().unwrap());
}
