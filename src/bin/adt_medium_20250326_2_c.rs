use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(isize, isize); n],
    }
    xy.push((0, 0));
    let (mut x, mut y) = (0, 0);
    let mut ans = 0.0;
    for &(x2, y2) in xy.iter() {
        ans += (((x-x2)*(x-x2) + (y-y2)*(y-y2)) as f64).sqrt();
        x = x2;
        y = y2;
    }
    println!("{}", ans);
}