use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }
    for &(x, y) in xy.iter() {
        let mut max_dist = 0;
        let mut ans = 0;
        for (j, &(x2, y2)) in xy.iter().enumerate() {
            let x_diff = x.abs_diff(x2);
            let y_diff = y.abs_diff(y2);
            let dist = x_diff*x_diff + y_diff*y_diff;
            if dist > max_dist {
                max_dist = dist;
                ans = j+1;
            }
        }
        println!("{}", ans);
    }
}