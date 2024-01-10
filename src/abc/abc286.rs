fn c() {
    input! {
      n:usize,
      a: usize,
      b:usize,
      s:Chars
    }

    let mut min_cost = n * b + n * a;
    for num_rotate in 0..n - 1 {
        let mut count_same = 0;
        for i in 0..n {
            let from_back = n - 1 - i;
            let index = (from_back + 2 * num_rotate) % n;
            if s[i] == s[index] {
                count_same += 1;
            }
        }
        let count_wrong = n - count_same;
        min_cost = std::cmp::min(min_cost, count_wrong / 2 * b + num_rotate * a);
    }
    println!("{}", min_cost);
    // 1: n,2,4,6,8,..,n-2,n
    // 2: n-1,1,3,5
}
