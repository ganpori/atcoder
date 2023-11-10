use proconio::input;

pub fn a() {
    input! {
        n:usize,
        s:String,
    }
    let result = s.find("ABC");
    match result {
        Some(value) => println!("{}", value + 1),
        None => println!("-1"),
    }
}

pub fn b() {
    input! {
      n:usize,
      m:usize,
      s:String,
      t:String,
    }
    let is_prefix = t.starts_with(s.as_str());
    let is_suffix = t.ends_with(s.as_str());
    if is_prefix && is_suffix {
        println!("0");
    } else if is_prefix && !is_suffix {
        println!("1");
    } else if !is_prefix && is_suffix {
        println!("2");
    } else if !is_prefix && !is_suffix {
        println!("3");
    }
}

pub fn c() {
    use std::collections::VecDeque;
    input! {
      n:usize,
      m:usize,
      a:[usize;m],
    }
    let mut next_firework = a[0];
    let mut a_queue = VecDeque::from(a);

    for day_i in 1..=n {
        if (next_firework as i32 - day_i as i32) >= 0 {
            // 最終日はnだからaが空になることは考慮しなくてよい
            println!("{}", next_firework - day_i);
        } else {
            while (next_firework as i32 - day_i as i32) < 0 {
                next_firework = a_queue.pop_front().unwrap();
            }
            println!("{}", next_firework - day_i);
        }
    }
}

pub fn d() {
    input! {n:usize,}
}
