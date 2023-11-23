fn a() {
    input! {
        n:usize
    }
    let s="3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

    for (i, si) in s.to_string().chars().enumerate() {
        print!("{}", si);
        if n + 1 == i {
            break;
        }
    }
}

// 実装が少しめんどくさいけど地道に頑張るだけ
fn b() {
    input! {
        n:usize
    }

    let mut vec_c = Vec::new();
    let mut vec_a = vec![];
    for i in 0..n {
        input! {c:usize}
        vec_c.push(c);
        input! {a:[usize;c]}
        vec_a.push(a);
    }
    input! {x:usize}

    let mut vec_tousen = vec![];
    let mut num_kake_min = 38;
    for i in 0..n {
        for value in &vec_a[i] {
            if x == *value {
                vec_tousen.push(i);
                num_kake_min = std::cmp::min(num_kake_min, vec_a[i].len());
                break;
            }
        }
    }

    let mut count = 0;
    for b in &vec_tousen {
        if vec_a[*b].len() == num_kake_min {
            count += 1;
        }
    }
    println!("{}", count);
    for b in &vec_tousen {
        if vec_a[*b].len() == num_kake_min {
            print!("{} ", *b + 1);
        }
    }
}
