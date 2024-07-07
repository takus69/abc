use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n],
    }
    let mut ans = a[0..k].to_vec().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    ans = format!("{} {}", ans, x);
    ans = format!("{} {}", ans, a[k..].to_vec().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    println!("{}", ans);
}