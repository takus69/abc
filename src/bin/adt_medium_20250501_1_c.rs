use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n-1],
    }
    let mut i = n-2;
    let mut ans = 1;
    while i > 0 {
        if p[i] > 1 {
            i = p[i]-2;
            ans += 1;
        } else {
            i = 0;
        }
    }
    println!("{}", ans);
}