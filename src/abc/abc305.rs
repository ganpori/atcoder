// (0..=100).step_by(5)で刻み幅5のイテレーション
fn a() {
    input! {
      n:isize,
    }

    let mut min = 5;
    let mut index = 0;
    for i in (0..=100).step_by(5) {
        if min > (n - i).abs() {
            min = (n - i).abs();
            index = i;
        }
    }

    println!("{}", index);
}

// charも順番を統合で比較できる
fn b() {
    input! {
      mut p:char,
      mut q:char,
    }

    if q < p {
        // unicodeポイントによって比較してる
        (p, q) = (q, p);
    }

    let dist = [
        ('A', 3),
        ('B', 1),
        ('C', 4),
        ('D', 1),
        ('E', 5),
        ('F', 9),
        ('G', 0),
    ];
    let mut start = false;
    let mut sum = 0;
    for i in 0..dist.len() {
        if p == dist[i].0 {
            start = true;
        } else if q == dist[i].0 {
            break;
        }
        if start {
            sum += dist[i].1;
        }
    }

    println!("{}", sum);
}

fn c() {
    input! {
      h:usize,
      w:usize,
      s:[Chars;h]
    }
    let mut h_cookie = vec![0; h];
    let mut w_cookie = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                h_cookie[i] += 1;
                w_cookie[j] += 1;
            }
        }
    }
    let h_max = h_cookie.iter().max().unwrap();
    let w_max = w_cookie.iter().max().unwrap();

    let mut h_i = 0;
    let mut w_j = 0;
    for i in 0..h {
        if h_cookie[i] == h_max - 1 {
            h_i = i;
        }
    }
    for j in 0..w {
        if w_cookie[j] == w_max - 1 {
            w_j = j;
        }
    }

    println!("{} {}", h_i + 1, w_j + 1);
}

// 割った値で考えるとわかりずらいからnの次元のママ考えるようにする
// 欲しい値を出しやすくなるような中間の関数を考えその演算から値を出すこと考える。
// ここだったらxまでに何分寝たか関数f(x)を考えf(r)-f(l)が欲しい値になる。
// なぜこれが簡単か？場合分けが減ってバグが減る？構造をうまく抽象的にとらえてるからそうなる？
fn d() {
    input! {
      n:usize,
      a:[usize;n],
      q:usize,
      lr:[[usize;2];q]
    }

    let mut cumsum_sleep = vec![0; n / 2];
    cumsum_sleep[0] = a[2] - a[1];
    for i in 1..n / 2 {
        let one_sleep = a[2 * i + 2] - a[2 * i + 1];
        cumsum_sleep[i] = cumsum_sleep[i - 1] + one_sleep;
    }
    dbg!(&cumsum_sleep);

    for i in 0..q {
        let start_time = lr[i][0];
        let end_time = lr[i][1];
        let start_index = binary_search(start_time, &a);
        let end_index = binary_search(end_time, &a);

        // dbg!(start_index, end_index);
        // 一気にやらずに　上のほうと下のほうで分けてから演算。
        // まず上のほうの整理
        if end_index == 0 {
            println!("0");
        } else {
            let mut total_sleep = cumsum_sleep[(end_index - 1) / 2];

            let last_sleep_diff = if end_index % 2 == 0 {
                a[end_index] - end_time
            } else {
                a[end_index + 1] - a[end_index]
            };
            total_sleep -= last_sleep_diff;

            // 上の整理ついたので下の整理する
            if start_index == 0 {
                println!("{}", total_sleep);
            } else {
                let first_sleep_diff = if start_index % 2 == 0 {
                    a[start_index] - start_time
                } else {
                    a[start_index + 1] - a[start_index]
                };

                // dbg!(total_sleep, last_sleep_diff, first_sleep_diff);
                total_sleep = total_sleep + first_sleep_diff - cumsum_sleep[(start_index - 1) / 2];

                println!("{}", total_sleep);
            }
        }
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
