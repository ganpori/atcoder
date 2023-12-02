// absをつけないで片側からまず抑える。その後別のifで逆方向から抑える。
//　引き算判定は足し算に置き換えて値の範囲が辺にならないように調整
fn d() {
    input! {
      n:usize,
      m:usize,
      d:isize,
      mut a:[isize;n],
      mut b:[isize;m]
    }
    // a.sort();
    b.sort();

    let mut max_value: isize = -1;
    for i in 0..n {
        let present_a = a[i];

        // b-a<=d判定。大きい方だけ探せばいいから逆側の端は調べなくてよい？？
        let min_false_index = b.partition_point(|&bi| bi <= d + present_a);

        if min_false_index == 0 {
        } else {
            // a-b<=d判定。
            if b[min_false_index - 1] + d >= present_a {
                let value = present_a + b[min_false_index - 1];

                max_value = std::cmp::max(max_value, value);
            }
        }
    }

    println!("{}", max_value);
}
