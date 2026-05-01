use proconio::input;
use ac_library::{LazySegtree, Monoid, MapMonoid};

struct Add;
impl Monoid for Add {
    type S = usize;
    fn identity() -> Self::S {
        0
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
    }
}

struct AddMap;
impl MapMonoid for AddMap {
    type M = Add;
    type F = usize;

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(&f: &Self::F, &x: &<Add as Monoid>::S) -> <Add as Monoid>::S {
        f + x
    }

    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    input! {
        n: usize,
    }
    let mut a = Vec::new();
    for i in 0..=(600000) {
        a.push(i);
    }
    let mut segtree: LazySegtree<AddMap> = a.into();

    // debug
    // let mut a = Vec::new();
    // for i in 0..=n {
    //     a.push(segtree.prod(i..(i+1)));
    // }
    // println!("{:?}", a);

    for i in 0..n {
        input! {
            l: usize,
            r: usize,
        }
        // 二分探索でインデックスを取得
        let mut ok = 0;
        let mut ng = 600000;
        while ok + 1 < ng {
            let m = (ok + ng) / 2;
            let v = segtree.prod(m..(m+1));
            if v < l {
                ok = m;
            } else {
                ng = m;
            }
        }
        let li = ok+1;
        let mut ok = 0;
        let mut ng = 600000;
        while ok + 1 < ng {
            let m = (ok + ng) / 2;
            let v = segtree.prod(m..(m+1));
            if v <= r {
                ok = m;
            } else {
                ng = m;
            }
        }
        let ri = ok;
        // println!("li: {}, ri: {}", li, ri);
        // 区間に1を加算
        segtree.apply_range(li..=ri, 1);

        // debug
        // let mut a = Vec::new();
        // for i in 0..=n {
        //     a.push(segtree.prod(i..(i+1)));
        // }
        // println!("{:?}", a);
    }
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            x: usize,
        }
        let ans = segtree.prod(x..(x+1));
        println!("{}", ans);
    }


    // debug
    // let mut a = Vec::new();
    // for i in 0..=n {
    //     a.push(segtree.prod(i..(i+1)));
    // }
    // println!("{:?}", a);
}