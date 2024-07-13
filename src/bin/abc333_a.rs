use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let ans = vec![n; n];
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}