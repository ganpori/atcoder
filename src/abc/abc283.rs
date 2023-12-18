fn a() {
    input! {
      a:usize,
      b:usize
    }
    let ans = a.pow(b as u32);
    print!("{}", ans);
}

fn b() {
    input! {
      n:usize,
     mut a:[usize;n],
      q:usize
    }

    for _ in 0..q {
        input! {
            q_type:usize
        }
        if q_type == 1 {
            input! {
                k:usize,
                x:usize
            }
            a[k - 1] = x;
        } else if q_type == 2 {
            input! {
                k:usize
            }
            println!("{}", a[k - 1]);
        }
    }
}

fn c() {
    input! {
     mut  s:Chars,
    }
    let mut count = 0;
    while s.len() > 1 {
        if s[s.len() - 1] == '0' && s[s.len() - 2] == '0' {
            s.pop();
            s.pop();
        } else {
            s.pop();
        }
        count += 1;
    }
    if s.len() == 1 {
        count += 1;
    }

    println!("{}", count);
}

fn d() {
    input! {
      s:Chars,
    }

    let mut index_latest_left_vec = vec![];
    let mut set_from_left_index_vec: Vec<HashSet<char>> = vec![];
    let mut set_hako: HashSet<char> = HashSet::new();
    for i in 0..s.len() {
        if s[i] == '(' {
            index_latest_left_vec.push(i);
            set_from_left_index_vec.push(HashSet::new());
        } else if s[i] == ')' {
            for key in set_from_left_index_vec[set_from_left_index_vec.len() - 1].iter() {
                set_hako.remove(key);
            }
            set_from_left_index_vec.pop();
            index_latest_left_vec.pop();
        } else {
            if set_hako.contains(&s[i]) {
                println!("No");
                return;
            }
            set_hako.insert(s[i]);
            let num = index_latest_left_vec.len();
            if num > 0 {
                set_from_left_index_vec[num - 1].insert(s[i]);
            }
        }
    }

    println!("Yes");
}
