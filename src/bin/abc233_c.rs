use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u128,
    }
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        input! {
            li: usize,
            ai: [usize; li],
        }
        a.push(ai);
    }
    println!("{}", dfs(1, 0, n, x, &a));

    fn dfs(v: u128, i: usize, t: usize, x: u128, a: &[Vec<usize>]) -> usize {
        if v == x && i == t { return 1; }
        if v > x || i == t { return 0; }
        let mut cnt = 0;
        for &ai in &a[i] {
            cnt += dfs(v*(ai as u128), i+1, t, x, a);
        }

        cnt
    }
}