fn a() {
    input! {
      n:usize,
      p:usize,
      q:usize,
      d:[usize;n]
    }

    let mut min_price = p;
    for i in 0..n {
        min_price = std::cmp::min(min_price, q + d[i]);
    }

    println!("{}", min_price);
}

fn b() {
    input! {
      n:usize,
      m:usize,
    }
    let mut price = vec![0; n];
    let mut command = vec![0; n];
    let mut function = vec![];
    for i in 0..n {
        input! {
            p:usize,
            c:usize,
            f:[usize;c]
        }
        price[i] = p;
        command[i] = c;
        function.push(f);
    }

    for i in 0..n {
        //iに対してjは上位互換であるか
        for j in 0..n {
            if price[j] <= price[i] && command[i] <= command[j] {
                let mut all_func = true;
                for func_i in &function[i] {
                    // func_iがfunc_jの中になかったら上位互換じゃない。全部あったらOK。
                    if !function[j].contains(func_i) {
                        all_func = false;
                    }
                }
                if (all_func && price[j] < price[i]) || (all_func && command[i] < command[j]) {
                    println!("Yes");
                    return;
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
    print!("No");
}

// 反転と通常状態どちらにある演算を行っても同じになる状態（標準状態）で保存しておくと単純
// pythonだと辞書順で小さい方というものが選べる
// vecでもstringでもhashセットそのままぶち込むこと可能
// 毎回反転させても時間的には大丈夫っぽい
// N=10^5であればO(N^2)も20msぐらいで演算可能。
// hashsetのunion,difference,intersection,symmetric_differenceとかめちゃ便利そう
// hashsetで含むか判定する演算はO(1)。
fn c() {
    input! {
      n:usize,
    mut  s:[Chars;n]
    }

    let mut set_str = HashSet::new();
    for i in 0..n {
        let mut s_i_reverse = s[i].clone();
        s_i_reverse.reverse();
        if !set_str.contains(&s[i]) || set_str.contains(&s_i_reverse) {
            set_str.insert(s_i_reverse);
        }
        // dbg!(&set_str);
    }

    println!("{}", set_str.len());
}
