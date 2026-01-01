use proconio::input;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let (mut opt_ans, mut opt_score) = greedy(n, &xy);
    eprintln!("greedy score: {}", opt_score);

    let (ans, score) = two_opt(n, &xy);
    if true && score < opt_score {
        opt_ans = ans;
        opt_score = score;
    }

    for i in 0..(n+1) {
        opt_ans[i] += 1;
    }
    println!("{}", opt_ans.iter().join(" "));
    eprintln!("opt score: {}", opt_score);
}

fn calc_score(ans: &Vec<usize>, xy: &Vec<(usize, usize)>) -> f64 {
    let mut score = 0.0;
    let mut pre = ans[0];
    for &i in ans.iter().skip(1) {
        score += dist(xy[pre], xy[i]);
        pre = i;
    }

    score
}

fn dist(xy1: (usize, usize), xy2: (usize, usize)) -> f64 {
    let (x1, y1) = xy1;
    let (x2, y2) = xy2;
    let x_diff = x1.abs_diff(x2) as f64;
    let y_diff = y1.abs_diff(y2) as f64;
    let dist = (x_diff*x_diff + y_diff*y_diff).sqrt();

    dist
}

fn two_opt(n: usize, xy: &Vec<(usize, usize)>) -> (Vec<usize>, f64) {
    let seed: u64 = 42;
    let mut rng = ChaCha20Rng::seed_from_u64(seed);

    let mut score = 0.0;
    let mut ans: Vec<usize> = vec![0];

    let mut pre = 0;
    for i in 1..n {
        ans.push(i);
        score += dist(xy[pre], xy[i]);
        pre = i;
    }
    ans.push(0);

    let mut improved_cnt = 0;
    for _ in 0..200000 {
        let choices: Vec<usize> = (1..n).choose_multiple(&mut rng, 2);
        let i1 = choices[0].min(choices[1]);
        let i2 = choices[0].max(choices[1]);

        let ii0 = ans[i1-1];
        let ii1 = ans[i1];
        let ii2 = ans[i2];
        let ii3 = ans[i2+1];
        let old_dist = dist(xy[ii0], xy[ii1]) + dist(xy[ii2], xy[ii3]);
        let new_dist = dist(xy[ii0], xy[ii2]) + dist(xy[ii1], xy[ii3]);
        if old_dist > new_dist {
            improved_cnt += 1;
            let mut old: Vec<usize> = ans[i1..=i2].to_vec();
            old.reverse();
            for (i, &ai) in old.iter().enumerate() {
                ans[i1+i] = ai;
            }
            // eprintln!("score: {}, dist: {} => {}", score, old_dist, new_dist);
            score -= old_dist - new_dist;
        }
    }
    
    let score = calc_score(&ans, &xy);
    eprintln!("score: {}, improved: {}", score, improved_cnt);

    (ans, score)
}

fn greedy(n: usize, xy: &Vec<(usize, usize)>) -> (Vec<usize>, f64) {
    let mut visited: Vec<bool> = vec![false; n];
    let mut ans: Vec<usize> = vec![0];
    let mut now_i = 0;
    let mut score = 0.0;
    for _ in 0..(n-1) {
        let mut nearest_i = 0;
        let mut min_dist = f64::MAX;
        for i in 1..n {
            if visited[i] { continue; }
            let dist = dist(xy[now_i], xy[i]);
            if dist < min_dist {
                min_dist = dist;
                nearest_i = i;
            }
        }
        now_i = nearest_i;
        ans.push(nearest_i);
        visited[nearest_i] = true;
        score += min_dist;
    }
    ans.push(0);

    (ans, score)
}