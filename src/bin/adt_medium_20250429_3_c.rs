use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut ans = a[0]-1;
    for &ai in a.iter() {
        if ans+1 == ai {
            ans = ai;
            continue;
        } else {
            println!("{}", ans+1);
            return;
        }
    }
}