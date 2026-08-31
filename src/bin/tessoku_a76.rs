use proconio::input;
use ac_library::FenwickTree;

fn main() {
    input! {
        n: usize,
        w: usize,
        l: usize,
        r: usize,
        mut x: [usize; n],
    }
    const MOD: isize = 1_000_000_007;
    x.push(0);
    x.push(w);
    x.sort();
    let mut reachable = FenwickTree::new(n+3, 0);
    reachable.add(0, 1);
    reachable.add(1, -1);
    for (i, &xi) in x.iter().enumerate() {
        if reachable.sum(0..=i) == 0 { continue; }
        let cnt = reachable.sum(0..=i);
        let li = match x.binary_search(&(xi+l)) {
            Ok(li) => { li },
            Err(li) => { li },
        };
        let ri = match x.binary_search(&(xi+r)) {
            Ok(ri) => { ri },
            Err(ri) => { ri-1 },
        };
        // println!("i: {}, li: {}, ri: {}, cnt: {}", i, li, ri, cnt);
        let cnt2 = reachable.sum(li..=li);
        reachable.add(li, -cnt2);
        reachable.add(li, (cnt+cnt2)%MOD);
        let cnt2 = reachable.sum(ri+1..=(ri+1));
        reachable.add(ri+1, -cnt2);
        reachable.add(ri+1, (MOD+cnt2-cnt)%MOD);
    }
    // for i in 0..(n+2) {
    //     print!("{} ", reachable.sum(0..=i)%MOD);
    // }
    println!("{}", reachable.sum(0..=(n+1))%MOD);
}