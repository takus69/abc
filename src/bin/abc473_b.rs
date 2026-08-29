use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    a.sort();
    let mut ans: usize = a.iter().sum();
    let mut i = 0;
    while i < n-1 {
        if a[i] == a[i+1] {
            ans -= a[i]*2;
            i += 2;
        } else {
            i += 1;
        }
    }

    println!("{}", ans);
}