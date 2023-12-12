fn a() {
    input! {
      n:usize,
      t:Chars
    }
    let mut num_t = 0;
    let mut num_a = 0;
    for i in 0..n {
        if t[i] == 'T' {
            num_t += 1;
        } else {
            num_a += 1;
        }
        if num_t * 2 >= n {
            println!("T");
            return;
        }
        if num_a * 2 >= n {
            println!("A");
            return;
        }
    }

    println!("{}", n);
}

fn b() {
    input! {
      n:usize,
      a:[isize;n]
    }

    for i in 0..n - 1 {
        print!("{} ", a[i]);
        let diff = a[i + 1] - a[i];
        if diff > 0 {
            for j in 1..diff {
                print!("{} ", a[i] + j);
            }
        } else if diff < 0 {
            for j in 1..-diff {
                print!("{} ", a[i] - j);
            }
        }
    }

    println!("{}", a[n - 1]);
}

fn c() {
    input! {
      s:Chars,
      t:Chars
    }

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();
    let mut sum_at = 0;
    for c in 'a'..'z' {
        map_s.insert(c, 0);
        map_t.insert(c, 0);
    }

    for i in 0..s.len() {
        *map_s.entry(s[i]).or_insert(0) += 1 as isize;
        *map_t.entry(t[i]).or_insert(0) += 1 as isize;
        if s[i] == '@' {
            sum_at += 1;
        }
        if t[i] == '@' {
            sum_at += 1
        }
    }

    for k in map_s.keys() {
        if *k != 'a'
            && *k != 't'
            && *k != 'c'
            && *k != 'o'
            && *k != 'd'
            && *k != 'e'
            && *k != 'r'
            && *k != '@'
        {
            if map_t.keys().contains(k) {
                if map_t[k] != map_s[k] {
                    print!("No");
                    return;
                }
            } else {
                print!("No");
                return;
            }
        }
    }
    let mut sum_diff: isize = 0;
    let vec_target = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for c in vec_target.iter() {
        sum_diff += (map_s[c] - map_t[c]).abs();
    }
    if sum_diff > sum_at {
        print!("No");
    } else {
        print!("Yes")
    }
}
