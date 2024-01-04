fn c() {
    input! {
      n:usize,
      k:usize,
      a:[usize;n]
    }

    let set_a: HashSet<usize> = a.clone().into_iter().collect();
    for i in 0..k {
        if !set_a.contains(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", k);
}
