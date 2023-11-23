fn a() {
    input! {
        s:Chars
    }

    for value in s {
        if value != 'a' && value != 'i' && value != 'u' && value != 'e' && value != 'o' {
            print!("{}", value);
        }
    }
}

fn b() {
    input! {
        m:usize,
        d:[usize;m]
    }

    let sum_d: usize = d.iter().sum();
    let middle_day = (sum_d + 1) / 2;

    let mut count_day = 0;
    for month in 1..=m {
        for day in 1..=d[month - 1] {
            count_day += 1;
            if count_day == middle_day {
                println!("{} {}", month, day);
                break;
            }
        }
    }
}

// 辺に最大値をとらずにhashmapの中身をvecにして要素を伸ばしていく方がわかりやすい
// その要素vecをsortするとその要素の0番目と1番目だけ見ればよい
// 別解。最大値のカップを決めてその種類を知っておく。残りn-1カップに対して判定してやればよい。
fn c() {
    input! {
        n:usize,
        fs:[[usize;2];n]
    }

    // ただの全列挙？O(N)オーダーでやれば問題なし

    let mut strongest_flavors = HashMap::new();
    // hashの中身は[最大値、二個目の値、最大値の出現回数]
    for fs_i in fs.iter() {
        let value_reference = strongest_flavors.entry(fs_i[0]).or_insert([fs_i[1], 0, 0]);
        if value_reference[0] < fs_i[1] {
            value_reference[1] = value_reference[0];
            value_reference[0] = fs_i[1];
            value_reference[2] = 1;
        } else if value_reference[0] == fs_i[1] && value_reference[2] == 0 {
            value_reference[2] += 1;
        } else if value_reference[1] < fs_i[1] {
            value_reference[1] = fs_i[1];
        }
    }
    // println!("{:?}", strongest_flavors);
    // 同じ味と異なる味のパターンで最大値を出す
    let mut value_1st = 0;
    let mut value_2nd = 0;
    let mut key_1st = 0;
    for (key, value) in &strongest_flavors {
        if value[0] >= value_1st {
            value_2nd = value_1st;
            value_1st = value[0];
            key_1st = *key;
        } else if value[0] > value_2nd {
            value_2nd = value[0];
        }
    }

    let strongest_second_value = strongest_flavors.get(&key_1st).unwrap()[1];
    // println!("{:?}", strongest_flavors);
    // println!("strongest_second: {}", strongest_second_value);
    // println!("key_1st: {}", key_1st);
    // println!("value_1st: {}", value_1st);
    // println!("value_2nd: {}", value_2nd);

    if value_1st + value_2nd > value_1st + strongest_second_value / 2 {
        println!("{}", value_1st + value_2nd);
    } else {
        println!("{}", value_1st + strongest_second_value / 2);
    }
}
