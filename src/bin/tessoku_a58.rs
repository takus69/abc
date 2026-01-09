use proconio::input;

pub struct SegmentTree {
    size: usize,
    data: Vec<isize>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n { size *= 2; }  // n 以上の最小の 2 のべき乗を求める
        let n_nodes = size*2;  // セグメント木全体を格納するために 2*size の領域を確保
        let data: Vec<isize> = vec![0; n_nodes];

        Self { size, data }
    }

    fn update(&mut self, i: usize, x: isize) {
        let mut pos = self.size + i;
        self.data[pos] = x;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos*2].max(self.data[pos*2+1]);
        }
    }

    fn query(&self, l: usize, r: usize) -> isize {
        self._query(l, r, 1, self.size, 1)
    }

    fn _query(&self, l: usize, r: usize, node_l: usize, node_r: usize, node: usize) -> isize {
        if r <= node_l || node_r <= l { return isize::MIN; }
        if l <= node_l && node_r <= r { return self.data[node]; }
        let node_m = (node_l + node_r) / 2;
        let ans_l = self._query(l, r, node_l, node_m, node*2);
        let ans_r = self._query(l, r, node_m, node_r, node*2+1);

        ans_l.max(ans_r)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut seg = SegmentTree::new(n+1);
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    p: usize,
                    x: isize,
                }
                seg.update(p, x);
            },
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", seg.query(l, r));
            },
            _ => {},
        }
    }
}