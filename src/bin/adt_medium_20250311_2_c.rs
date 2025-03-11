use proconio::input;

fn main() {
    input! {
        a: [usize; 64],
    }
    let mut ans: usize = 0;
    for (i, &ai) in a.iter().enumerate() {
        if ai == 1 {
            ans += 1 << i;
        }
    }
    println!("{}", ans);
}