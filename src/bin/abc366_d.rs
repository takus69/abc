use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n*n],
        q: usize,
        lr: [(usize, usize, usize, usize, usize, usize); q],
    }

    let mut sum_a = a.clone();
    // z方向の累積和
    for i in 0..(n*n) {
        for z in 1..n {
            sum_a[i][z] += sum_a[i][z-1];
        }
    }
    // y方向の累積和
    for z in 0..n {
        for x in 0..n {
            for y in 1..n {
                sum_a[x*n+y][z] += sum_a[x*n+y-1][z];
            }
        }
    }
    // x方向の累積和
    for z in 0..n {
        for y in 0..n {
            for x in 1..n {
                sum_a[x*n+y][z] += sum_a[(x-1)*n+y][z];
            }
        }
    }
    fn sum_i(x: usize, y: usize, z: usize, sum_a: &Vec<Vec<usize>>) -> usize {
        let n = sum_a[0].len();
        if x == 0 || y == 0 || z == 0 {
            return 0;
        }
        let ri = (x-1)*n + (y-1);
        sum_a[ri][z-1]
    }
    for (lx, rx, ly, ry, lz, rz) in lr {
        let ri = (rx-1)*n + (ry-1);
        let mut ans = sum_i(rx, ry, rz, &sum_a);  // sum_a[ri][rz-1];
        let lx = lx-1;
        let ly = ly-1;
        let lz = lz-1;
        ans += sum_i(lx, ly, rz, &sum_a);
        ans += sum_i(rx, ly, lz, &sum_a);
        ans += sum_i(lx, ry, lz, &sum_a);
        ans -= sum_i(lx, ry, rz, &sum_a);
        ans -= sum_i(rx, ly, rz, &sum_a);
        ans -= sum_i(rx, ry, lz, &sum_a);
        ans -= sum_i(lx, ly, lz, &sum_a);
        println!("{}", ans);
    }
}