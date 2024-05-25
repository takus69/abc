use proconio::input;
use ac_library::floor_sum;

fn main() {
    input! {
        t: i64,
        q: [(i64, i64, i64, i64); t],
    }
    
    for (n, m, a, b) in q {
        let ans = floor_sum(n, m, a, b);
        println!("{}", ans);
    }
}