use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }
    let mut comsum: Vec<usize> = vec![0];
    let mut sum_a = 0;
    for &ai in &a {
        sum_a += ai;
        comsum.push(sum_a);
    }
    // x超を2分探索
    let mut ng = 0;
    let mut ok = (x/sum_a+1)*n;
    while ng+1 < ok {
        let m = (ok+ng)/2;
        let value = (m/n)*sum_a + comsum[m%n];
        if value > x {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}