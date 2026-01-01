use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut visited: Vec<bool> = vec![false; n];
    let mut ans: Vec<usize> = vec![1];
    let mut now_i = 0;
    let mut score = 0.0;
    for _ in 0..(n-1) {
        let mut nearest_i = 0;
        let mut min_dist = f64::MAX;
        let (x1, y1) = xy[now_i];
        for i in 1..n {
            if visited[i] { continue; }
            let (x2, y2) = xy[i];
            let x_diff = x1.abs_diff(x2) as f64;
            let y_diff = y1.abs_diff(y2) as f64;
            let dist = (x_diff*x_diff + y_diff*y_diff).sqrt();
            if dist < min_dist {
                min_dist = dist;
                nearest_i = i;
            }
        }
        now_i = nearest_i;
        ans.push(nearest_i+1);
        visited[nearest_i] = true;
        score += min_dist;
    }
    ans.push(1);

    for a in ans.iter() {
        println!("{}", a);
    }

    // println!("score: {}", score);
}