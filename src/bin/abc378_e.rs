use proconio::input;
use ac_library::FenwickTree;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut sum_a = Vec::new();
    let mut prefix_sum = 0;
    sum_a.push(prefix_sum);
    for ai in a.iter() {
        prefix_sum += ai;
        prefix_sum %= m;
        sum_a.push(prefix_sum);
    }
    let mut sum_a2 = Vec::new();
    let mut prefix_sum = 0;
    for &si in sum_a.iter() {
        prefix_sum += si;
        sum_a2.push(prefix_sum);
    }
    let mut ft: FenwickTree<usize> = FenwickTree::new(m+1, 0);

    let mut ans = 0;
    for r in 1..=n {
        ft.add(sum_a[r], 1);
        ans += sum_a[r]*r + m*ft.sum((sum_a[r]+1)..(m+1)) - sum_a2[r-1];
    }

    println!("{}", ans);
}