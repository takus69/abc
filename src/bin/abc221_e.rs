use proconio::input;
use ac_library::{FenwickTree, ModInt998244353};

type Mint = ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ft = FenwickTree::new(n, Mint::new(0));
    let mut a2: Vec<(usize, usize)> = Vec::new();
    let mut pow2 = vec![Mint::new(1); n+1];
    for i in 0..n {
        a2.push((a[i], i));
        pow2[i+1] = pow2[i]*2;
    }
    a2.sort();
    for i in 0..n {
        let (_, j) = a2[i];
        if j == 0 { continue; }
        ft.add(i, pow2[j-1]); 
    }
    let mut ans = Mint::new(0);
    for (i, &ai) in a.iter().enumerate() {
        let pos = a2.binary_search(&(ai, i)).unwrap();
        let v = ft.sum(pos..=pos);
        ft.add(pos, -v);
        let cnt = ft.sum(pos..n);
        ans += cnt/pow2[i];
    }
    println!("{}", ans.val());
}