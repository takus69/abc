use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }
    let mut cnt = 0;
    let target = (d.iter().sum::<usize>()+1)/2;
    for mi in 0..m {
        for di in 1..=d[mi] {
            cnt += 1;
            if cnt == target {
                println!("{} {}", mi+1, di);
            }
        }
    }
}