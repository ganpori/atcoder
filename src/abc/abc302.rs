fn a() {
    input! {
        a:usize,
        b:usize
    }
    let mut val = a / b;
    if a % b != 0 {
        val += 1;
    }
    println!("{}", val);
}

fn b() {
    input! {
        h:isize,
        w:isize,
        s:[Chars;h],
    }

    let list_diff = [
        [[0, 0], [0, 1], [0, 2], [0, 3], [0, 4]],
        [[0, 0], [1, 0], [2, 0], [3, 0], [4, 0]],
        [[0, 0], [1, 1], [2, 2], [3, 3], [4, 4]],
        [[0, 0], [-1, 1], [-2, 2], [-3, 3], [-4, 4]],
        [[0, 0], [0, -1], [0, -2], [0, -3], [0, -4]],
        [[0, 0], [-1, 0], [-2, 0], [-3, 0], [-4, 0]],
        [[0, 0], [-1, -1], [-2, -2], [-3, -3], [-4, -4]],
        [[0, 0], [1, -1], [2, -2], [3, -3], [4, -4]],
    ];

    for i in 0..h {
        for j in 0..w {
            for diff in list_diff {
                let mut is_snuke = true;
                for (k, c) in ['s', 'n', 'u', 'k', 'e'].iter().enumerate() {
                    let next_h = i + diff[k][0];
                    let next_w = j + diff[k][1];
                    if 0 <= next_h && next_h <= h - 1 && 0 <= next_w && next_w <= w - 1 {
                        if *c == s[next_h as usize][next_w as usize] {
                        } else {
                            is_snuke = false;
                        }
                    } else {
                        is_snuke = false;
                    }
                }
                if is_snuke {
                    for k in 0..5 {
                        println!("{} {}", i + diff[k][0] + 1, j + diff[k][1] + 1)
                    }
                    return;
                }
            }
        }
    }
}

// absをつけないで片側からまず抑える。その後別のifで逆方向から抑える。
//　引き算判定は足し算に置き換えて値の範囲が辺にならないように調整
fn d() {
    input! {
      n:usize,
      m:usize,
      d:isize,
      mut a:[isize;n],
      mut b:[isize;m]
    }
    // a.sort();
    b.sort();

    let mut max_value: isize = -1;
    for i in 0..n {
        let present_a = a[i];

        // b-a<=d判定。大きい方だけ探せばいいから逆側の端は調べなくてよい？？
        let min_false_index = b.partition_point(|&bi| bi <= d + present_a);

        if min_false_index == 0 {
        } else {
            // a-b<=d判定。
            if b[min_false_index - 1] + d >= present_a {
                let value = present_a + b[min_false_index - 1];

                max_value = std::cmp::max(max_value, value);
            }
        }
    }

    println!("{}", max_value);
}
