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

fn b() {
    input! {
        n:usize,
        t:usize,
        c:[usize;n],
        r:[usize;n]
    }
    let mut map = HashMap::new();
    for i in 0..n {
        let entry = map.entry(c[i]).or_insert(Vec::new());
        entry.push([r[i], i]);
    }
    if map.contains_key(&t) {
        let a = map.get_mut(&t).unwrap();
        a.sort();
        println!("{}", a[a.len() - 1][1] + 1);
    } else {
        let a = map.get_mut(&c[0]).unwrap();
        a.sort();
        println!("{}", a[a.len() - 1][1] + 1);
    }
}
// インタラクティブにやるときはproconioとstdinをいい感じにしないといけない
// inputのところも変更必要
fn d() {
    use proconio::{input, marker::Chars, source::line::LineSource};
    use std::io::{stdin, BufReader};
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
      from &mut source,
      n:usize,
    }

    let mut left = 0;
    let mut right = n - 1;
    let mut middle = (left + right) / 2;
    while left < right {
        println!("? {}", middle + 1);
        input! {
        from &mut source,
        value:usize}
        if value == 1 {
            right = middle;
        } else if value == 0 {
            left = middle + 1;
        }
        middle = (left + right) / 2;
    }
    println!("! {}", middle);
}
