use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt: Vec<usize> = vec![0; 101];
    for &ai in a.iter() {
        cnt[ai] += 1;
    }
    let mut ans = 0;
    for &ci in cnt.iter() {
        if ci >= 3 {
            ans += ci*(ci-1)*(ci-2)/6
        }
    }
    println!("{}", ans);
}