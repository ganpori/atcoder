fn a() {
    input! {
      m:usize,
      d:usize,
      year:usize,
      month:usize,
      day:usize
    }

    let next_day = if day + 1 > d { 1 } else { day + 1 };
    let next_month = if day + 1 > d && month + 1 > m {
        1
    } else if day + 1 > d {
        month + 1
    } else {
        month
    };
    let next_year = if month + 1 > m && day + 1 > d {
        year + 1
    } else {
        year
    };

    println!("{} {} {}", next_year, next_month, next_day);
}

fn b() {
    input! {
      n:usize,
      s:usize,
      m:usize,
      l:usize
    }

    let max_s = (n / 6) + 1;
    let max_m = (n / 8) + 1;
    let max_l = (n / 12) + 1;

    let mut price = 10000000;
    for i in 0..=max_s {
        for j in 0..=max_m {
            for k in 0..=max_l {
                if i * 6 + j * 8 + k * 12 >= n {
                    price = std::cmp::min(price, i * s + j * m + k * l);
                }
            }
        }
    }

    // dbg!(&kouritu, &num_min_set);
    println!("{}", price);
}

// rustのbinary_searchは同じ値のindexをとってこれるからめっちゃ有能。これを使えるようにする。

fn c() {
    input! {
      n:usize,
      a:[usize;n]
    }
    let mut b = vec![[0; 2]; n];
    for i in 0..n {
        b[i] = [a[i], i];
    }
    b.sort();

    let mut d = a.clone();
    d.sort();

    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(b[i][0]).or_insert(0) += 1;
    }

    let mut cumusum = vec![0; n];
    cumusum[0] = b[0][0];
    for i in 1..n {
        cumusum[i] = cumusum[i - 1] + b[i][0];
    }

    for i in 0..n {
        let target_value = a[i];
        let target_index_in_sort = binary_search(target_value, &d);

        let result = cumusum[n - 1]
            - cumusum[target_index_in_sort]
            - target_value * (count[&target_value] - 1);
        print!("{} ", result);
    }
}

fn binary_search(value: usize, array: &Vec<usize>) -> usize {
    let mut l = 0;
    let mut r = array.len() - 1;
    let mut middle = (l + r) / 2;
    while l < r {
        if array[middle] > value {
            r = middle;
        } else if array[middle] < value {
            l = middle + 1;
        } else if array[middle] == value {
            r = middle;
        }
        middle = (l + r) / 2;
    }
    middle
}
