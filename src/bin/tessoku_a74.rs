use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [[usize; n]; n],
    }
    let mut pos: Vec<(usize, usize)> = vec![(0, 0); n+1];
    for i in 0..n {
        for j in 0..n {
            if p[i][j] > 0 {
                pos[p[i][j]] = (i, j);
            }
        }
    }
    let mut ans = 0;
    // 縦を移動
    for k in 1..=n {
        let (mut i, j) = pos[k];
        while i+1 > k {
            for j2 in 0..n {
                if p[i-1][j2] > 0 {
                    let k2 = p[i-1][j2];
                    pos[k] = (i-1, j);
                    pos[k2] = (i, j2);
                    p[i][j] = 0;
                    p[i-1][j2] = 0;
                    p[i-1][j] = k;
                    p[i][j2] = k2;
                    ans += 1;
                    i -= 1;
                    break;
                }
            }
        }
        // println!("pos: {:?}", pos);
    }
    // 横を移動
    for k in 1..=n {
        let (i, mut j) = pos[k];
        while j+1 > k {
            for i2 in 0..n {
                if p[i2][j-1] > 0 {
                    let k2 = p[i2][j-1];
                    pos[k] = (i, j-1);
                    pos[k2] = (i2, j);
                    p[i][j] = 0;
                    p[i2][j-1] = 0;
                    p[i][j-1] = k;
                    p[i2][j] = k2;
                    ans += 1;
                    j -= 1;
                    break;
                }
            }
        }
        // println!("pos: {:?}", pos);
    }
    println!("{}", ans);
}