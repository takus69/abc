use proconio::input;

fn main() {
    input! {
        mut a: [usize; 4],
    }
    a.sort();
    let mut ans = 0;
    let mut i = 0;
    while i < 3 {
        if a[i] == a[i+1] {
            ans += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    println!("{}", ans);
}