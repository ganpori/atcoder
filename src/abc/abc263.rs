fn a() {
    input! {
        abcde:[usize;5]
    }
    let s: HashSet<usize> = HashSet::from_iter(abcde.clone());
    let mut count = 1;
    for i in 1..5 {
        if abcde[i] == abcde[0] {
            count += 1;
        }
    }
    if (count == 2 || count == 3) && s.len() == 2 {
        println!("Yes");
    } else {
        print!("No");
    }
}

fn b() {
    input! {
            n:usize,
        p:[usize;n-1]
    }

    let mut parent_vec = vec![];
    parent_vec.push(p[n - 2]);
    while *parent_vec.last().unwrap() != 1 {
        let parent = *parent_vec.last().unwrap();
        let grandparet = p[parent - 2];
        parent_vec.push(grandparet);
    }
    println!("{}", parent_vec.len());
}
