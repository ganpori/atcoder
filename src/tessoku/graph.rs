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

pub fn a62() {
    input! {
        n:usize,
        m:usize,
        vec_edge:[[usize;2];m]
    }
    let mut adjacency_list = vec![Vec::<usize>::new(); n];
    for edge in vec_edge.iter() {
        adjacency_list[edge[0] - 1].push(edge[1] - 1);
        adjacency_list[edge[1] - 1].push(edge[0] - 1);
    }

    // println!("{:?}", adjacency_list);
    let mut vec_visited = vec![false; n];
    dfs(0, &mut vec_visited, &adjacency_list);

    if vec_visited.iter().all(|x| *x) {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}

fn dfs(pos: usize, vec_visited: &mut Vec<bool>, adjacency_list: &Vec<Vec<usize>>) {
    vec_visited[pos] = true;
    for adjacency_node in adjacency_list[pos].iter() {
        if vec_visited[*adjacency_node] {
            continue;
        } else {
            dfs(*adjacency_node, vec_visited, adjacency_list) //ここをmut とか&つけなくていいのなぜかわからん
        }
    }
}
