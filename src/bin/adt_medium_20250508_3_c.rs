use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ans1 = 0;
    for i in 0..n {
        if a[i] == b[i] {
            ans1 += 1;
        }
    }
    let mut ans2 = 0;
    for ai in a.iter() {
        if b.contains(ai) {
            ans2 += 1;
        }
    }
    ans2 -= ans1;

    println!("{}", ans1);
    println!("{}", ans2);
}