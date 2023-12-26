fn a() {
    input! {
      s:Chars,
    }

    for i in 0..s.len() {
        if s[i].is_ascii_uppercase() {
            println!("{}", i + 1);
            return;
        }
    }
}

fn b() {
    input! {
      n:usize,
     mut  x:[usize;5*n]
    }
    x.sort();
    let mut sum = 0;

    for i in n..4 * n {
        sum += x[i];
    }
    println!("{}", sum as f64 / (3.0 * n as f64));
}

fn c() {
    input! {
      n:usize,
      s:Chars
    }
    let mut trajectory_set = HashSet::new();
    let mut pos = [0, 0];
    trajectory_set.insert(pos);
    for query in s {
        if query == 'R' {
            pos[0] += 1;
        } else if query == 'L' {
            pos[0] -= 1;
        } else if query == 'U' {
            pos[1] += 1;
        } else if query == 'D' {
            pos[1] -= 1;
        }

        if trajectory_set.contains(&pos) {
            println!("Yes");
            return;
        } else {
            trajectory_set.insert(pos);
        }
    }
    println!("No");
}
