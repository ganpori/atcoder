fn a() {
    input! {
        n:usize,
        m:usize,
        p:usize
    }
    if n >= m {
        let num_moon = (n - m) / p + 1;
        println!("{}", num_moon);
    } else {
        println!("0");
    }
}

fn b() {
    input! {
        n:usize,
        vec_rectangle:[[usize;4];n]
    }

    let mut count_cordinate: [[usize; 100]; 100] = [[0; 100]; 100];
    for rectangle in vec_rectangle {
        for x in rectangle[0]..rectangle[1] {
            for y in rectangle[2]..rectangle[3] {
                count_cordinate[x][y] += 1;
            }
        }
    }
    // println!("{:?}", count_cordinate);
    let mut count = 0;
    for x in 0..100 {
        for y in 0..100 {
            if count_cordinate[x][y] >= 1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
