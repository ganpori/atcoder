fn c() {
    input! {
      n:usize,
      ab:[[usize;2];n]
    }

    let mut g: HashMap<usize, Vec<usize>> = HashMap::new();
    for edge in ab {
        g.entry(edge[0]).or_insert(vec![]).push(edge[1]);
        g.entry(edge[1]).or_insert(vec![]).push(edge[0]);
    }
    // dbg!(&g);

    let mut dist: HashMap<usize, usize> = HashMap::new();
    for key in g.keys() {
        dist.insert(*key, n + 1);
    }
    let mut q = VecDeque::new();

    dist.insert(1, 0);
    q.push_back(1);
    let mut max_floor = 1;
    while q.len() > 0 {
        let pos: usize = q.pop_front().unwrap();
        // dbg!(pos);
        max_floor = std::cmp::max(max_floor, pos);
        if g.contains_key(&pos) {
            for next_pos in g.get(&pos).unwrap() {
                if *dist.get(&next_pos).unwrap() == n + 1 {
                    q.push_back(*next_pos);
                    dist.insert(*next_pos, *dist.get(&pos).unwrap());
                }
            }
        }
    }
    println!("{}", max_floor);
}
