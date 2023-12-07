fn a() {
    input! {
        n:usize,
        s:Chars
    }
    let mut flag = false;

    for i in 0..n {
        if s[i] == '|' && flag == false {
            flag = true;
        } else if flag && s[i] == '*' {
            println!("in");
            return;
        } else if flag && s[i] == '|' {
            break;
        }
    }

    println!("out");
}
