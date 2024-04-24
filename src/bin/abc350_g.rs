use proconio::input;
use std::collections::HashSet;

fn decription(q: (usize, usize, usize), xk: usize, n: usize) -> (usize, usize, usize) {
    let r#mod = 998244353;
    let a = 1 + (q.0*(1+xk) % r#mod) % 2;
    let b = 1 + (q.1*(1+xk) % r#mod) % n;
    let c = 1 + (q.2*(1+xk) % r#mod) % n;
    (a, b, c)
    //q
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind{ parent, size }
    }

    fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.root(self.parent[i]);
            self.size[i] = self.size[self.parent[i]];
            self.parent[i]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.root(self.parent[x]);
        let y_root = self.root(self.parent[y]);
        let x_size = self.size(x_root);
        let y_size = self.size(y_root);
        if x_root != y_root {
            self.parent[y_root] = x_root;
            self.size[x_root] = x_size + y_size;
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.root(self.parent[x]);
        let y_root = self.root(self.parent[y]);
        x_root == y_root
    }

    fn size(&mut self, i: usize) -> usize {
        let i_root = self.root(i);
        self.size[i_root]
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        query: [(usize, usize, usize); q],
    }

    let mut uf = UnionFind::new(n);
    let mut xk = 0;
    let mut edge: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut par: Vec<isize> = vec![-1; n];

    fn dfs(edge: &Vec<HashSet<usize>>, par: &mut Vec<isize>, root: usize, child: usize) {
        // println!("root: {}, par: {:?}, edge: {:?}", root, par, edge[root]);
        par[child] = root as isize;
        for i in &edge[child] {
            if i == &root { continue; }
            dfs(edge, par, child, *i);
        }
    }

    for q in query {
        let (a, b, c) = decription(q, xk, n);
        let bi = b - 1;
        let ci = c - 1;
        // println!("{} {} {}", a, b, c);
        
        if a == 1 {
            // 木の構築
            let mut i1 = bi;
            let mut i2 = ci;
            if uf.size(bi) > uf.size(ci) {
                i1 = ci;
                i2 = bi;
            }
            dfs(&edge, &mut par, i2, i1);
            // println!("{:?}", par);
            edge[bi].insert(ci);
            edge[ci].insert(bi);
            uf.unite(bi, ci);
        } else {
            // 出力
            // println!("{:?}", par);
            let mut ans = -1;
            if uf.same(bi, ci) && par[bi] == par[ci] {
                ans = par[bi];
            } else if uf.same(bi, ci) && par[bi] != -1 && par[par[bi] as usize] == ci as isize {
                ans = par[bi];
            } else if uf.same(bi, ci) && par[ci] != -1 && par[par[ci] as usize] == bi as isize {
                ans = par[ci];
            }
            xk = (ans + 1) as usize;
            println!("{}", xk);
        }
        // println!("edge: {:?}", edge);
    }
}