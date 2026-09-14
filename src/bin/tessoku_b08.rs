use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
    }
    let mut prefix: Vec<Vec<isize>> = vec![vec![0; 1502]; 1502];
    for &(x, y) in &xy {
        prefix[x][y] += 1;
    }
    for x in 0..=1500 {
        for y in 0..=1500 {
            prefix[x][y+1] += prefix[x][y];
        }
    }
    for y in 0..=1500 {
        for x in 0..=1500 {
            prefix[x+1][y] += prefix[x][y];
        }
    }
    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        let ans = prefix[c][d] - prefix[c][b-1] - prefix[a-1][d] + prefix[a-1][b-1];
        println!("{}", ans);
    }
}