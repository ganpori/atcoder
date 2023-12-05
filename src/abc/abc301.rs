fn a() {
    input! {
      n:usize,
      t:Chars
    }
    let mut num_t = 0;
    let mut num_a = 0;
    for i in 0..n {
        if t[i] == 'T' {
            num_t += 1;
        } else {
            num_a += 1;
        }
        if num_t * 2 >= n {
            println!("T");
            return;
        }
        if num_a * 2 >= n {
            println!("A");
            return;
        }
    }

    println!("{}", n);
}

fn b() {
    input! {
      n:usize,
      a:[isize;n]
    }

    for i in 0..n - 1 {
        print!("{} ", a[i]);
        let diff = a[i + 1] - a[i];
        if diff > 0 {
            for j in 1..diff {
                print!("{} ", a[i] + j);
            }
        } else if diff < 0 {
            for j in 1..-diff {
                print!("{} ", a[i] - j);
            }
        }
    }

    println!("{}", a[n - 1]);
}
