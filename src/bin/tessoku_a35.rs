use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
    for j in 0..n {
        dp.insert((n, j), a[j]);
    }
    for i in (1..n).rev() {
        for j in 0..=i {
            let dp1 = dp.get(&(i+1, j)).unwrap();
            let dp2 = dp.get(&(i+1, j+1)).unwrap_or(dp1);
            if i%2==1 {
                dp.insert((i, j), *(dp1.max(dp2)));
            } else {
                dp.insert((i, j), *(dp1.min(dp2)));
            }
        }
    }
    println!("{}", dp.get(&(1, 0)).unwrap());
}