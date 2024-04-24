fn primes(n: usize) -> Vec<usize> {
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

fn pow(x: usize, n: usize) -> usize {
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

fn modint(x: usize, n: usize, r#mod: usize) -> usize {
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
}