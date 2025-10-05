use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v: Vec<usize> = vec![1; n+1];
    v[0] = 0;
    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        }
        let mut cnt = 0;
        if v[x] > 0 {
            for i in (1..=x).rev() {
                if v[i] == 0 { break; }
                cnt += v[i];
                v[i] = 0;
            }
            v[y] += cnt;
        }
        println!("{}", cnt);
    }
}