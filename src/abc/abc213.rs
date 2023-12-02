// 座標の値を何番目に小さいかの順番に変換する->座標圧縮
fn c() {
    input! {
      h:usize,
      w:usize,
      n:usize,
    }
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        input! {
            ai:usize,
            bi:usize
        }
        a[i] = ai;
        b[i] = bi;
    }

    let mut a_new = a.clone();
    let mut b_new = b.clone();
    a_new.sort();
    b_new.sort();
    a_new.dedup();
    b_new.dedup();

    let mut map_a = HashMap::new();
    let mut map_b = HashMap::new();
    for i in 0..a_new.len() {
        map_a.insert(a_new[i], i);
    }
    for i in 0..b_new.len() {
        map_b.insert(b_new[i], i);
    }

    for i in 0..n {
        println!("{} {}", map_a[&a[i]] + 1, map_b[&b[i]] + 1);
    }
}
