fn a() {
    input! {
      n:usize,
      s:Chars
    }
    if s.contains(&'x') {
        print!("No");
        return;
    }
    if s.contains(&'o') {
        print!("Yes");
    } else {
        print!("No")
    }
}

fn b() {
    input! {
      n:usize,
      a:[[usize;n];n],
      b:[[usize;n];n],
    }

    let mut rot_a = a.clone();
    for _ in 0..4 {
        let rot_a_before = rot_a.clone();
        rot_a = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                rot_a[i][j] = rot_a_before[n - j - 1][i];
            }
        }
        // dbg!(&rot_a, &rot_a_before);
        let mut is_same = true;
        for i in 0..n {
            for j in 0..n {
                if rot_a[i][j] == 1 && b[i][j] == 0 {
                    is_same = false;
                }
                if !is_same {
                    break;
                }
            }
            if !is_same {
                break;
            }
        }
        if is_same {
            print!("Yes");
            return;
        }
    }

    println!("No");
}

fn c() {
    input! {
      n:usize,
      q:usize,
    }
    let mut map_hako = HashMap::new();
    let mut map_num = HashMap::new();
    for _ in 0..q {
        input! {
          type_query: usize,
          i:usize
        }
        if type_query == 1 {
            input! {j:usize}
            map_hako.entry(j).or_insert(BinaryHeap::new()).push(i);
            map_num.entry(i).or_insert(BTreeSet::new()).insert(j);
        } else if type_query == 2 {
            // print!("2: ");
            let a = map_hako[&i].clone();
            for value in a.into_sorted_vec() {
                print!("{} ", value);
            }
            println!();
        } else if type_query == 3 {
            // print!("3: ");
            let a = map_num[&i].clone();
            for value in a.iter() {
                print!("{} ", value);
            }
            println!();
        }
        // dbg!(&map_hako, &map_num);
    }
}
