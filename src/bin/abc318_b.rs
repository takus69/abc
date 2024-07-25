use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut cover: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
    for (a, b, c, d) in abcd {
        for i in a..b {
            for j in c..d {
                cover[i][j] = true;
            }
        }
    }
    let mut ans = 0;
    for i in 0..100 {
        for j in 0..100 {
            if cover[i][j] { ans += 1; }
        }
    }
    println!("{}", ans);
}