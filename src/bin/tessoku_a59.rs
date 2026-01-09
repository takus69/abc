use proconio::input;

pub struct SegmentTree<M: Monoid> {
    size: usize,
    data: Vec<M::T>,
    _marker: std::marker::PhantomData<M>,  // 型Mを使っていることを明示
}

impl<M: Monoid> SegmentTree<M> {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n { size *= 2; }  // n 以上の最小の 2 のべき乗を求める
        let n_nodes = size*2;  // セグメント木全体を格納するために 2*size の領域を確保
        let data: Vec<M::T> = vec![M::identify(); n_nodes];

        Self { size, data, _marker: std::marker::PhantomData }
    }

    fn update(&mut self, i: usize, x: M::T) {
        let mut pos = self.size + i;
        self.data[pos] = x;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = M::operate(self.data[pos*2], self.data[pos*2+1]);
        }
    }

    fn query(&self, l: usize, r: usize) -> M::T {
        self._query(l, r, 1, self.size, 1)
    }

    fn _query(&self, l: usize, r: usize, node_l: usize, node_r: usize, node: usize) -> M::T {
        if r <= node_l || node_r <= l { return M::identify(); }
        if l <= node_l && node_r <= r { return self.data[node]; }
        let node_m = (node_l + node_r) / 2;
        let ans_l = self._query(l, r, node_l, node_m, node*2);
        let ans_r = self._query(l, r, node_m, node_r, node*2+1);

        M::operate(ans_l, ans_r)
    }
}

pub trait Monoid {
    type T: Copy;
    fn identify() -> Self::T;
    fn operate(a: Self::T, b: Self::T) -> Self::T;
}

pub struct Sum;

impl Monoid for Sum {
    type T = isize;

    fn identify() -> Self::T {
        0
    }

    fn operate(a: Self::T, b: Self::T) -> Self::T {
        a+b
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut segtree: SegmentTree<Sum> = SegmentTree::new(n+1);
    for _ in 0..q {
        input! {
            c: usize,
            a: usize,
            b: usize,
        }
        if c == 1 {
            segtree.update(a, b as isize);
        } else {
            let v = segtree.query(a, b);
            println!("{}", v);
        }
    }
}