use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort();
    let mut ans = 0;
    let mut pre_y = usize::MAX;
    for &(x, y) in &xy {
        if y < pre_y {
            ans += 1;
            pre_y = y;
        }
    }
    println!("{}", ans);
}