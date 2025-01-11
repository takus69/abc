use proconio::input;
use ac_library::{LazySegtree, Monoid, MapMonoid};

struct Add;
impl Monoid for Add {
    type S = i64;
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
    type F = i64;

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
        a: [i64; n],
    }
    let mut segtree: LazySegtree<AddMap> = a.into();
    let mut ans = Vec::new();
    for i in 1..n {
        let ai = segtree.prod((i-1)..i);
        let pay = ai.min((n-i) as i64);
        segtree.apply_range(i..(i+pay as usize), 1);
        ans.push(ai-pay);
    }
    ans.push(segtree.prod((n-1)..n));
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}