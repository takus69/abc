use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        mut x: usize,
    }
    let sum_a: usize = a.iter().sum();
    let mut ans = x / sum_a;
    x -= ans * sum_a;
    ans *= n;
    for &ai in a.iter() {
        if x >= ai {
            x -= ai;
            ans += 1;
        } else {
            ans += 1;
            break;
        }
    }
    println!("{}", ans);
}