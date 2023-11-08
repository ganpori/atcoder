use proconio::input;

pub fn a61() {
    input! {
        n:usize,
        m:usize,
        ab:[[usize;2];m]
    }

    let mut adjacent_list = vec![Vec::<usize>::new(); n];
    for edge in ab.iter() {
        adjacent_list[edge[0] - 1].push(edge[1] - 1);
        adjacent_list[edge[1] - 1].push(edge[0] - 1);
    }

    for (i, node) in adjacent_list.iter().enumerate() {
        let mut str_output = format!("{}{}", (i + 1).to_string(), ": {");
        let s = node
            .iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{}: {{{}}}", i + 1, s);
    }
}
