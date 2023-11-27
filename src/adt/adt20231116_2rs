// strから数値への変換はparseよりto_digitの方がわかりよい
fn a() {
    input! {
      x:String,
    }

    let mut integer_str = String::new();
    let mut integer: usize = 0;
    for (i, c) in x.chars().enumerate() {
        if c == '.' {
            let value = x.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
            if value >= 5 {
                println!("{}", integer + 1);
            } else {
                println!("{}", integer);
            }
            return;
        } else {
            integer_str.push(c);
            integer = integer_str.parse().unwrap();
        }
    }
}

fn b() {
    input! {
      card:[usize;5],
    }
    let mut set_card = HashSet::new();
    let num_init = card[0];
    let mut sum = 0;
    for i in 0..5 {
        set_card.insert(card[i]);
        if card[i] == num_init {
            sum += 1;
        }
    }
    if set_card.len() == 2 && (sum == 2 || sum == 3) {
        println!("Yes");
    } else {
        println!("No");
    }
}
