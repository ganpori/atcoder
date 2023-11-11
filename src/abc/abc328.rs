use proconio::input;

pub fn a() {
    input! {
      n:usize,
      x:usize,
      s:[usize;n]
    }

    let mut sum_s = 0;
    for s_value in s.iter() {
        if *s_value <= x {
            sum_s += s_value;
        }
    }
    println!("{}", sum_s);
}
