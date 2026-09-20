use proconio::input;
use itertools::Itertools;

pub struct SegmentTree<M: Monoid> {
    size: usize,
    data: Vec<(M::T, usize)>,
    _marker: std::marker::PhantomData<M>,  // 型Mを使っていることを明示
}

impl<M: Monoid> SegmentTree<M> {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n { size *= 2; }  // n 以上の最小の 2 のべき乗を求める
        let n_nodes = size*2;  // セグメント木全体を格納するために 2*size の領域を確保
        let data: Vec<(M::T, usize)> = vec![M::identify(); n_nodes];

        Self { size, data, _marker: std::marker::PhantomData }
    }

    fn update(&mut self, i: usize, x: M::T) {
        let mut pos = self.size + i;
        self.data[pos] = (x, i);
        while pos > 1 {
            pos /= 2;
            self.data[pos] = M::operate(self.data[pos*2], self.data[pos*2+1]);
        }
    }

    fn query(&self, l: usize, r: usize) -> (M::T, usize) {
        self._query(l, r, 0, self.size, 1)
    }

    fn _query(&self, l: usize, r: usize, node_l: usize, node_r: usize, node: usize) -> (M::T, usize) {
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
    fn identify() -> (Self::T, usize);
    fn operate(a: (Self::T, usize), b: (Self::T, usize)) -> (Self::T, usize);
}

pub struct Max;

impl Monoid for Max {
    type T = isize;

    fn identify() -> (Self::T, usize) {
        (isize::MIN, 0)
    }

    fn operate(a: (Self::T, usize), b: (Self::T, usize)) -> (Self::T, usize) {
        if a.0 > b.0 {
            a
        } else {
            b
        }
    }
}

pub struct Min;

impl Monoid for Min {
    type T = isize;

    fn identify() -> (Self::T, usize) {
        (isize::MAX, 0)
    }

    fn operate(a: (Self::T, usize), b: (Self::T, usize)) -> (Self::T, usize) {
        if a.0 < b.0 {
            a
        } else {
            b
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [isize; n],
        lr: [(usize, usize); m],
    }
    let mut max_seg: SegmentTree<Max> = SegmentTree::new(n+1);
    let mut min_seg: SegmentTree<Min> = SegmentTree::new(n+1);
    for (i, &pi) in p.iter().enumerate() {
        max_seg.update(i+1, pi);
        min_seg.update(i+1, pi);
    }
    for i in 1..=n {
        let (v, j) = max_seg.query(i, i+1);
    }
    for &(l, r) in &lr {
        let (max_v, max_i) = max_seg.query(l, r+1);
        let (min_v, min_i) = min_seg.query(l, r+1);
        max_seg.update(max_i, min_v);
        max_seg.update(min_i, max_v);
        min_seg.update(max_i, min_v);
        min_seg.update(min_i, max_v);
    }
    let mut ans: Vec<isize> = Vec::new();
    for i in 1..=n {
        let (v, _) = max_seg.query(i, i+1);
        ans.push(v);
    }
    println!("{}", ans.iter().join(" "));
}