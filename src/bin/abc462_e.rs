use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: isize,
            b: isize,
            x: isize,
            y: isize,
        }
        let dist_x = x.abs();
        let dist_y = y.abs();
        let min_dist = dist_x.min(dist_y);
        let max_dist = dist_x.max(dist_y);
        let (small, large) = if a < b { (a, b) } else { (b, a) };
        // println!("small: {}, large: {}, min_dist: {}, max_dist: {}", small, large, min_dist, max_dist);
        // 同数移動
        let mut ans = 2*min_dist*small;
        // println!("ans1: {}", ans);
        // 残距離
        let diff = max_dist - min_dist;
        ans += (diff/2) * (small + large.min(3*small));
        // println!("ans2: {}", ans);
        ans += if diff%2==1 {
            if (a < b && dist_x > dist_y) || (a >= b && dist_x < dist_y) { small } else { large.min(3*small) }
        } else { 0 };
        // println!("ans3: {}", ans);
        println!("{}", ans);
    }
}