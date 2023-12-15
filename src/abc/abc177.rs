// unionfind
fn d() {
    input! {
      n:usize,
      m:usize,
      ab:[[usize;2];m]
    }
    let mut uf = UnionFind::new(n);
    for pair in ab {
        uf.unite(pair[0] - 1, pair[1] - 1);
    }

    let max_size = uf.size.iter().max().unwrap();
    println!("{}", max_size);
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
