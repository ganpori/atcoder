fn c() {
    input! {
      n:usize,
      m:usize
    }
    let mut s = vec![];
    for _ in 0..m {
        input! {ci:usize,
        ai:[usize;ci]}
        s.push(ai);
    }

    let mut count = 0;
    // 0b1から0b100000(m桁)までの間のループ
    for bit_expression in 0b1..(0b1usize.rotate_left(m as u32)) {
        let mut num_set = HashSet::new();
        // bitに対応するデータを集める
        for rotate_digit in 0..m {
            if 0b01 == (0b1 & bit_expression.rotate_right(rotate_digit as u32)) {
                for a in &s[rotate_digit] {
                    num_set.insert(a);
                }
            }
        }

        // num_setで数が足りてたらおｋ
        let mut ok = true;
        for i in 1..=n {
            if !num_set.contains(&i) {
                ok = false;
            }
        }
        if ok {
            count += 1;
        }
    }
    println!("{}", count);
}
