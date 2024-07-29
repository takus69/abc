use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [String; n],
    }
    let mut ans = "Yes";
    let mut pre: String = "".to_string();
    for i in 0..(s.len()-1) {
        let si = &s[i];
        if pre == "sweet" && si == "sweet" {
            ans = "No";
        }
        pre = si.clone();
    }
    println!("{}", ans);
}