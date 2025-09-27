use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans: isize = 0;
    for i in 1..=n {
        let i = i as isize;
        ans += if i%2==1 { -1 } else { 1 } *i*i*i;
    }
    println!("{}", ans);
}