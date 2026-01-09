use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        xy: [(usize, usize); q],
    }
    for i in 0..n {
        a[i] -= 1;
    }
    let mut dp: Vec<Vec<usize>> = Vec::new();
    dp.push(a.clone());
    for i in 0..30 {
        let mut tmp: Vec<usize> = Vec::new();
        for j in 0..n {
            tmp.push(dp[i][dp[i][j]])
        }
        dp.push(tmp);
    }

    for &(x, y) in xy.iter() {
        let mut x = x-1;
        for i in (0..30).rev() {
            if (y >> i)&1 == 1 {
                // println!("x: {}, y: {}, i: {}", x, y, i);
                x = dp[i][x];
            }
        }
        println!("{}", x+1);
    }
    // println!("dp: {:?}", dp);
}