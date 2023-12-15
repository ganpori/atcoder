fn d() {
    input! {
      n:usize,
      a:[usize;n]
    }
    let mut uf = UnionFind::new(n);

    let mut map_group = HashMap::<usize, Vec<usize>>::new();
    for (i, a_value) in a.iter().enumerate() {
        map_group.entry(*a_value).or_insert(Vec::new()).push(i);
    }

    for key in map_group.keys() {
        let group = &map_group[key];
        for index in 1..group.len() {
            uf.unite(group[index - 1], group[index]);
        }
    }

    let mut count_unite = 0;
    for i in 0..n {
        if uf.is_same(i, n - 1 - i) {
            continue;
        } else {
            uf.unite(i, n - 1 - i);
            count_unite += 1;
        }
    }
    println!("{}", count_unite);
}

struct UnionFind {
    size: Vec<usize>,
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            size: vec![1; n],
            parent: vec![n; n],
        }
    }

    fn root(&self, x: usize) -> usize {
        let mut parent_x = self.parent[x];
        if parent_x != self.parent.len() {
            parent_x = self.root(parent_x);
            return parent_x;
        }
        return x;
    }
    fn is_same(&self, x: usize, y: usize) -> bool {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x == root_y {
            true
        } else {
            false
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x == root_y {
            return;
        }
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
    }
}
