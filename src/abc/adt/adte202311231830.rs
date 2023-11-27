fn a() {
    input! {
      s:Chars,
    }
    print!("0");
    for i in 0..s.len() - 1 {
        print!("{}", s[i]);
    }
}

fn b() {
    input! {
      n:usize,
      m:usize,
      a:[usize;n],
      b:[usize;m]
    }

    let mut tokuten = 0;
    for n_monme in b {
        tokuten += a[n_monme - 1];
    }
    println!("{}", tokuten);
}

// クエリを実行しながら進むとわかりやすい。一気に受け取ると操作不要になるパターンもある。
fn c() {
    input! {
      n:usize,
    mut  a:[usize;n],
    q:usize,
    }

    for i in 0..q {
        input! {
            query_type:usize
        }
        if query_type == 1 {
            input! {
                k:usize,
                x:usize
            }
            a[k - 1] = x;
        } else if query_type == 2 {
            input! {
                k:usize
            }
            println!("{}", a[k - 1]);
        }
    }
}

// 知っているかどうかをtrue,falseの配列にしておくほうがよりわかりやすい
fn d() {
    input! {
      n:usize,
      x:usize,
      a:[usize;n]
    }

    let mut secret_group = HashSet::new();
    let mut secret_man = x - 1;
    secret_group.insert(x - 1);
    loop {
        let num_default = secret_group.len();
        let secret_man_next = a[secret_man] - 1;
        secret_group.insert(secret_man_next);
        if num_default == secret_group.len() {
            println!("{}", num_default);
            return;
        } else {
            secret_man = secret_man_next;
        }
    }
}

// setで含むかどうかの判定で実装したほうが早い。
// queueでpopとかしながらの強みは途中の状態を保存できること
fn e() {
    input! {
      n:usize,
      m:usize,
      s:[Chars;n],
      t:[Chars;m]
    }
    let mut s_queue = VecDeque::from(s);
    let mut t_queue = VecDeque::from(t);
    for i in 0..n {
        let eki = s_queue.pop_front().unwrap();
        if t_queue[0] == eki {
            t_queue.pop_front();
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
