use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut j = 0;
    for i in 1..=n {
        println!("{}", a[j]-i);
        if i == a[j] {
            j += 1;
        }
    }
}