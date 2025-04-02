use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut ans = 0;
    a.sort();
    for &ai in a.iter() {
        if ans < ai {
            break;
        } else if ans == ai {
            ans += 1;
        }
    }
    println!("{}", ans);
}