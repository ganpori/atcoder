use std::collections::HashSet;

#[allow(unused)]
use proconio::{input, marker::Chars};

fn main() {
    c();
}

fn c() {
    input! {
      n:usize,
      m:usize,
      uv:[[usize;2];m]
    }
    let mut uf = UnionFind::new(n);

    for edge in uv {
        uf.unite(edge[0] - 1, edge[1] - 1);
    }

    let mut s = HashSet::new();
    for i in 0..n {
        s.insert(uf.root(i));
    }
    println!("{}", s.len());
}

// #[derive(Debug)]のアトリビュートはdebugトレイトの自動実装を行うらしい。
// これでdbg!(&uf); println!("{:?}", uf);どちらも可能になる。
#[derive(Debug)]
struct UnionFind {
    size: Vec<usize>,
    parent: Vec<usize>,
    root_status: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            size: vec![1; n],
            parent: vec![n; n], // 点数はn。parentの値はparentのノードのindexを表す.->0..=n-1しか存在しない。nであるときは自分自身がrootであることを示す。
            root_status: (0..n).into_iter().collect(), //この変数の型は?
        }
    }

    fn root(&mut self, x: usize) -> usize {
        let mut parent_x = self.parent[x];
        if parent_x != self.parent.len() {
            parent_x = self.root(parent_x);
            self.root_status[x] = parent_x;
            return parent_x;
        }

        return x;
    }
    fn is_same(&self, x: usize, y: usize) -> bool {
        let root_x = self.root_status[x];
        let root_y = self.root_status[y];
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
            self.root_status[x] = self.root_status[y];
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.root_status[y] = self.root_status[x];
            self.size[root_x] += self.size[root_y];
        }
    }
    fn get_root_size_vec(&self) -> Vec<[usize; 2]> {
        // 呼び出すタイミングによってvecのサイズが変わるのでめっちゃ不安定な挙動する。
        // root_size_vecに所有権は移動していないっぽい
        //　一要素目が根、２要素目が根のサイズを表す。
        // hasumapを返すと受け手側でもhashmapを使わないといけないからデフォルトで使いやすいvecにする。
        let mut root_size_vec = vec![];
        for i in 0..self.parent.len() {
            if self.parent[i] == self.parent.len() {
                root_size_vec.push([i, self.size[i]]);
            }
        }
        return root_size_vec;
    }
}
