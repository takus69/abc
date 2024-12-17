use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(isize, isize); n],
    }
    xy.push((0, 0));
    let mut ans: f64 = 0.0;
    let (mut now_x, mut now_y) = (0, 0);
    for &(x, y) in xy.iter() {
        ans += (((now_x-x)*(now_x-x) + (now_y-y)*(now_y-y)) as f64).sqrt();
        (now_x, now_y) = (x, y);
    }
    println!("{}", ans);
}