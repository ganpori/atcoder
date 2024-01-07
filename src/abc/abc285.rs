fn c() {
    input! {s:Chars}

    let to_num_value = 64;

    let mut sum: usize = 0;
    for i in 0..s.len() {
        let num = s[i] as u8 - to_num_value;

        sum += num as usize * 26_usize.pow((s.len() - i - 1) as u32);
    }
    print!("{}", sum);
}
