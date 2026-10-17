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
        self._query(l, r, 0, self.size, 1)
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

pub struct Min;

impl Monoid for Min {
    type T = isize;

    fn identify() -> Self::T {
        isize::MAX
    }

    fn operate(a: Self::T, b: Self::T) -> Self::T {
        a.min(b)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        l: [usize; n-1],
        r: [usize; n-1],
    }
    let mut l_seg: SegmentTree<Min> = SegmentTree::new(n);
    let mut r_seg: SegmentTree<Min> = SegmentTree::new(n);
    for i in 0..(n-1) {
        l_seg.update(i+1, l[i] as isize);
        r_seg.update(i, r[i] as isize);
    }
    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        }
        let mut ans = if s < t {
            r[s-1]
        } else {
            l[s-2]
        };
        let tmp = if s < t {
            if s == 1 { usize::MAX } else {
                l[s-2] + r_seg.query(0, s-1) as usize
            }
        } else {
            if s == n { usize::MAX } else {
                r[s-1] + l_seg.query(s, n) as usize
            }
        };
        ans = ans.min(tmp);
        println!("{}", ans);
    }
}