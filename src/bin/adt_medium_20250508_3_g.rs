use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[[usize; n]; n]; n],
        q: usize,
    }
    // 累積和の計算
    let mut sum: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; n+1]; n+1]; n+1];
    for x in 1..=n {
        for y in 1..=n {
            for z in 1..=n {
                sum[x][y][z] = sum[x-1][y][z] + sum[x][y-1][z] + sum[x][y][z-1] + a[x-1][y-1][z-1] - sum[x][y-1][z-1] - sum[x-1][y][z-1] - sum[x-1][y-1][z] + sum[x-1][y-1][z-1];
            }
        }
    }

    for _ in 0..q {
        input! {
            lx: usize,
            rx: usize,
            ly: usize,
            ry: usize,
            lz: usize,
            rz: usize,
        }
        let ans = sum[rx][ry][rz] + sum[rx][ly-1][lz-1] + sum[lx-1][ry][lz-1] + sum[lx-1][ly-1][rz] - sum[lx-1][ry][rz] - sum[rx][ly-1][rz] - sum[rx][ry][lz-1] - sum[lx-1][ly-1][lz-1];
        println!("{}", ans);
    }
}