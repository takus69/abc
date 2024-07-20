use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut ans = -1;
    for i in 0..(n-2) {
        if &s[i..(i+3)] == "ABC" {
            ans = (i+1) as i32;
            break;
        }
    }
    println!("{}", ans);
}