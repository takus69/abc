use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    let mut sum_a = 0;
    let mut sum_b = 0;
    let mut ans = n;
    for i in 0..n {
        let ai = a[i];
        let bi = b[i];
        sum_a += ai;
        sum_b += bi;
        if sum_a > x {
            ans = i+1;
            break;
        }
        if sum_b > y {
            ans = i+1;
            break;
        }
    }
    println!("{}", ans);
}