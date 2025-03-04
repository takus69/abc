use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
        mut k: usize,
    }
    let mut ans: Vec<String> = Vec::new();
    x.reverse();
    if x.len() < k { println!("0"); std::process::exit(0); }
    let mut flg = false;
    for i in 0..k {
        let xi = x[i];
        let mut xi: usize = xi.to_string().parse().unwrap();
        if flg { xi += 1; }
        if xi >= 5 { flg = true; } else { flg = false; }
        ans.push("0".to_string());
    }
    if k == x.len() && flg { ans.push("1".to_string()); }
    for i in k..x.len() {
        let xi = x[i];
        let mut xi: usize = xi.to_string().parse().unwrap();
        if flg {
            xi += 1;
            if xi < 10 { flg = false } else { xi = 0; };
        }
        let xi = format!("{}", xi);
        ans.push(xi);
    }
    if k < x.len() && flg { ans.push("1".to_string()); }
    ans.reverse();
    let ans = ans.join("");
    let ans: usize = ans.parse().unwrap();
    println!("{}", ans);
}