use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut a_max = *(a.last().unwrap());
    for i in 0..(n-1) {
        let ai = a[i];
        a[i] += ((a_max - ai)/k)*k;
    }
    a.sort();
    let mut ans = a_max - a[0];
    for i in 0..(n-1) {
        let a_max = a_max.max(a[i]+k);
        ans = ans.min(a_max-a[i+1]);
    }
    println!("{}", ans);
}