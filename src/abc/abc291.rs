fn a() {
    input! {
      s:Chars,
    }

    for i in 0..s.len() {
        if s[i].is_ascii_uppercase() {
            println!("{}", i + 1);
            return;
        }
    }
}
