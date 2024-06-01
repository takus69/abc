use std::collections::{HashMap, VecDeque};

pub fn primes(n: usize) -> Vec<usize> {
    // エラトステネスの篩にて、n以下の素数リストを作成
    let mut is_prime: Vec<bool> = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=n {
        for j in ((i*2)..=n).step_by(i) {
            is_prime[j] = false;
        }
    }
    (0..=n).filter(|&i| is_prime[i]).collect()
}

pub fn pow(x: usize, n: usize) -> usize {
    // 繰り返し二乗法
    let mut ret = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ret *= x;
        }
        x *= x;
        n >>= 1;
    }
    ret
}

pub fn modint(x: usize, n: usize, r#mod: usize) -> usize {
    // modを取りながら繰り返し二乗法
    let mut ret = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            ret *= x;
            ret %= r#mod;
        }
        x *= x;
        x %= r#mod;
        n >>= 1;
    }
    ret
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind{ parent, size }
    }

    pub fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            self.parent[i] = self.root(self.parent[i]);
            self.size[i] = self.size[self.parent[i]];
            self.parent[i]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.root(self.parent[x]);
        let y_root = self.root(self.parent[y]);
        let x_size = self.size(x_root);
        let y_size = self.size(y_root);
        if x_root != y_root {
            self.parent[y_root] = x_root;
            self.size[x_root] = x_size + y_size;
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.root(self.parent[x]);
        let y_root = self.root(self.parent[y]);
        x_root == y_root
    }

    pub fn size(&mut self, i: usize) -> usize {
        let i_root = self.root(i);
        self.size[i_root]
    }
}

/// 幅優先探索を行う
/// # 引数
/// * `start` - 探索の開始位置
/// * `vertices` - 頂点の集合
/// * `edges` - 辺の集合.隣接リストとして保持
/// # 戻り値
/// * `path` - startから各頂点までの距離
/// * `prev` - startから各頂点までの移動において、各頂点の一つ前の頂点
pub fn bfs(start: usize, vertices: Vec<usize>, edges: HashMap<usize, Vec<usize>>) -> (HashMap<usize, usize>, HashMap<usize, i64>) {
    let mut path: HashMap<usize, usize> = HashMap::new();
    let mut que: VecDeque<usize> = VecDeque::new();
    let mut visited: HashMap<usize, bool> = HashMap::new();
    let mut prev: HashMap<usize, i64> = HashMap::new();

    // 初期化
    let max_len = vertices.len();
    for v in vertices.iter() {
        visited.insert(*v, false);
        path.insert(*v, max_len);
    }

    que.push_back(start);
    visited.insert(start, true);
    path.insert(start, 0);
    prev.insert(start, -1);
    while let Some(current) = que.pop_front() {
        let p = path[&current];
        if !edges.contains_key(&current) {
            continue;
        }
        for next in edges[&current].iter() {
            if !visited[next] {
                que.push_back(*next);
                visited.insert(*next, true);
                path.insert(*next, p + 1);
                prev.insert(*next, current as i64);
            }
        }
    }

    (path, prev)
}

/// 幅優先探索で取得したstartから各頂点までの距離をもとに、startからendまでの経路を復元する
/// # 引数
/// * `end` - 探索の終了位置
/// * `path` - bfsから取得した、startから各頂点までの距離
/// * `prev` - bfsから取得した、startから各頂点までの移動において、各頂点の一つ前の頂点
/// # 戻り値
/// * `path2` - startからendまでの最短経路の頂点.到達不可の場合はNone
pub fn reconstruct_path(end: usize, path: HashMap<usize, usize>, prev: HashMap<usize, i64>) -> Option<Vec<usize>> {
    let max_len = prev.len();
    let mut path2 = Vec::new();
    // startからendに到達できない場合はNoneを返却
    if path[&end] >= max_len { return None }

    let mut current = end;
    path2.push(current);
    while prev[&current] >= 0 {
        current = prev[&current] as usize;
        path2.push(current);
    }
    path2.reverse();

    Some(path2)
}

/// bit全探索で要素が使われる数を数え上げる
pub fn bit_manipulation(el_num: usize) -> usize {
    let mut cnt = 0;
    for mask in 0..(1<<el_num) {  // bit全探索で組合せを順に確認
        for i in 0..el_num {  // i番目の要素がつかわれるかを確認
            cnt += mask>>i & 1;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        assert_eq!(primes(6), vec![2, 3, 5]);
        assert_eq!(primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(3, 2), 9);
    }

    #[test]
    fn test_modint() {
        const MOD: usize = 998244353;
        assert_eq!(modint(10, 9, MOD), 10usize.pow(9) % MOD);
    }

    #[test]
    fn test_union_find() {
        let n = 5;
        let mut uf = UnionFind::new(n);
        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(1), 1);
        assert_eq!(uf.root(2), 2);
        assert_eq!(uf.root(3), 3);
        assert_eq!(uf.root(4), 4);
        assert_eq!(uf.size(0), 1);
        uf.unite(0, 2);
        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(2), 0);
        assert_eq!(uf.size(0), 2);
        assert_eq!(uf.size(2), 2);
        uf.unite(1, 3);
        uf.unite(3, 4);
        assert_eq!(uf.root(1), 1);
        assert_eq!(uf.root(3), 1);
        assert_eq!(uf.root(4), 1);
        assert_eq!(uf.size(1), 3);
        uf.unite(3, 4);
        assert_eq!(uf.size(1), 3);
        assert!(uf.same(0, 2));
        assert!(uf.same(1, 4));
    }

    #[test]
    fn test_bfs() {
        let v = vec![1, 3, 5, 7, 9];
        let mut e = HashMap::new();
        e.insert(1, vec![3, 9]);
        e.insert(3, vec![5, 1, 9]);
        e.insert(5, vec![7]);
        e.insert(7, vec![]);
        e.insert(9, vec![1, 5]);
        let (path, _) = bfs(1, v.clone(), e.clone());
        assert_eq!(path[&1], 0);
        assert_eq!(path[&3], 1);
        assert_eq!(path[&9], 1);
        assert_eq!(path[&5], 2);
        assert_eq!(path[&7], 3);
        assert_eq!(path[&9], 1);

        let (path, prev) = bfs(3, v, e);
        assert_eq!(path[&1], 1);
        assert_eq!(path[&3], 0);
        assert_eq!(path[&5], 1);
        assert_eq!(path[&7], 2);
        assert_eq!(path[&9], 1);

        if let Some(path2) = reconstruct_path(7, path, prev) {
            assert_eq!(path2[0], 3);
            assert_eq!(path2[1], 5);
            assert_eq!(path2[2], 7);
        } else {
            panic!();
        };

        let v = vec![0, 1, 2];
        let mut e = HashMap::new();
        e.insert(0, vec![1]);
        e.insert(1, vec![2]);
        let (path, prev) = bfs(0, v, e);
        if let Some(path2) = reconstruct_path(2, path, prev) {
            assert_eq!(path2[0], 0);
            assert_eq!(path2[1], 1);
            assert_eq!(path2[2], 2);
        } else {
            panic!();
        }

        let v = vec![0, 1, 2];
        let mut e = HashMap::new();
        e.insert(0, vec![1]);
        let (path, prev) = bfs(0, v.clone(), e.clone());
        if let Some(path2) = reconstruct_path(1, path.clone(), prev.clone()) {
            assert_eq!(path2[0], 0);
            assert_eq!(path2[1], 1);
        } else {
            panic!();
        }
        if let Some(_path2) = reconstruct_path(2, path, prev) {
            panic!();
        }
        let (path, prev) = bfs(2, v, e);
        if let Some(path2) = reconstruct_path(2, path.clone(), prev.clone()) {
            assert_eq!(path2[0], 2);
        } else {
            panic!();
        }
        if let Some(_path2) = reconstruct_path(1, path, prev) {
            panic!();
        }
    }

    #[test]
    fn test_bit_manipulation() {
        let cnt = bit_manipulation(1);
        assert_eq!(cnt, 1);
        let cnt = bit_manipulation(2);
        assert_eq!(cnt, 4);
        let cnt = bit_manipulation(3);
        assert_eq!(cnt, 12);
        let cnt = bit_manipulation(4);
        assert_eq!(cnt, 32);
    }
}