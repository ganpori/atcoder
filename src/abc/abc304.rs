//  タプルで文字列と数値を同時に受け取ることも可能。
//  sa:[(String,usize);n]
// 変な要素でも比較する方法。下記の場合(String,usize)型のusizeが最小の要素が得られる。
// let min_age = sa.iter().min_by_key(|&x| x.1).unwrap();
fn a() {
    input! {
      n:usize,
      sa:[(String,usize);n]
    }

    let (index_min, sa_min) = sa
        .iter()
        .enumerate()
        .min_by_key(|(i, element)| element.1)
        .unwrap();
    for i in index_min..n {
        println!("{}", sa[i].0);
    }
    for i in 0..index_min {
        println!("{}", sa[i].0);
    }
}

fn b() {
    input! {
        n:Chars
    }
    let len_n = n.len();

    if len_n <= 3 {
        for i in 0..len_n {
            print!("{}", n[i]);
        }
    } else {
        for i in 0..=2 {
            print!("{}", n[i]);
        }
        for i in 3..len_n {
            print!("0");
        }
    }
}

fn c() {
    input! {
        n:usize,
        d:usize,
        xy:[[isize;2];n]
    };

    let d_square = d.pow(2) as isize;

    let mut g = vec![Vec::<usize>::new(); n];
    for i in 0..n - 1 {
        //全組み合わせをいい感じに作る
        for j in i + 1..n {
            let dist_square = (xy[i][0] - xy[j][0]).pow(2) + (xy[i][1] - xy[j][1]).pow(2);
            if d_square >= dist_square {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }

    let mut visited = vec![false; n];
    dfs(0, &mut visited, &g);
    for is_sick in visited {
        if is_sick {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(node: usize, visited: &mut Vec<bool>, g: &Vec<Vec<usize>>) {
    visited[node] = true;
    for next_node in &g[node] {
        if visited[*next_node] {
            continue;
        } else {
            dfs(*next_node, visited, g);
        }
    }
}
