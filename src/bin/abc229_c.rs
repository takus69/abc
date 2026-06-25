use proconio::input;

fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by(|x, y| y.0.cmp(&x.0));
    let mut ans = 0;
    for &(ai, bi) in &ab {
        if bi < w {
            w -= bi;
            ans += ai*bi;
        } else {
            ans += ai*w;
            break;
        }
    }
    println!("{}", ans);
}