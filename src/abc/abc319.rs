fn a() {
    input! {
        s:String
    }
    if s == "tourist" {
        println!("3858");
    } else if s == "ksun48" {
        println!("3679");
    } else if s == "Benq" {
        println!("3658");
    } else if s == "Um_nik" {
        println!("3648");
    } else if s == "apiad" {
        println!("3638");
    } else if s == "Stonefeang" {
        println!("3630");
    } else if s == "ecnerwala" {
        println!("3613");
    } else if s == "mnbvmar" {
        println!("3555");
    } else if s == "newbiedmy" {
        println!("3516");
    } else if s == "semiexp" {
        println!("3481");
    }
}

fn b() {
    input! {
        n:usize
    }
    let mut c = '-';
    for j in 1..=9 {
        if n % j == 0 {
            c = std::char::from_digit(j as u32, 10).unwrap();
            break;
        }
    }
    print!("{}", c);

    for i in 1..=n {
        let mut c = '-';
        for j in 1..=9 {
            if n % j == 0 && i % (n / j) == 0 {
                c = std::char::from_digit(j as u32, 10).unwrap();
                break;
            }
        }
        print!("{}", c);
    }
}
