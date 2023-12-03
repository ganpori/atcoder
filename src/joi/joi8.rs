fn b() {
    input! {
       d:usize,
       n:usize,
       m:usize,
      mut dn:[usize;n -1],
      mut k:[usize;m]
    }
    dn.push(0);
    dn.sort();
    k.sort();

    // dbg!(&dn, &k);
    let mut sum_dist = 0;
    for i in 0..m {
        let point = dn.partition_point(|pos| pos <= &k[i]);
        // point=0になることはない。k>=0かつd[0]=0だから。
        if point == n {
            sum_dist += std::cmp::min(d - k[i], k[i] - dn[n - 1]);
        } else {
            sum_dist += std::cmp::min(dn[point] - k[i], k[i] - dn[point - 1]);
        }
    }
    println!("{}", sum_dist);
}
