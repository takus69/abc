use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    }
    xy.sort_by(|a, b| (a.0*(b.1-1), b.0*b.0+b.1*b.1).cmp(&((a.1-1)*b.0, a.0*a.0+a.1*a.1)));
    // println!("xy: {:?}", xy);
    let mut ans = 1;
    let mut pre_x = xy[0].0;
    let mut pre_y = xy[0].1-1;
    for i in 1..n {
        let (x, y) = (xy[i].0-1, xy[i].1);
        // println!("({}, {}), ({}, {}), y1*x2: {}, y2*x1: {}", pre_x, pre_y, x, y, pre_y*x, y*pre_x);
        if pre_y*x >= y*pre_x {
            ans += 1;
            pre_x = xy[i].0;
            pre_y = xy[i].1-1;
        }
    }
    println!("{}", ans);
}