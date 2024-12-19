use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }
    let mut ans = usize::MAX;
    // i回A円払ってから、B円の回数を確認する    
    for i in 0..=(n/2) {
        let mut tmp = a*i;
        for j in i..(n/2+i) {
            if s[j] != s[(n+i-(j-i)-1)%n] {
                tmp += b;
            }
        }
        ans = ans.min(tmp);

    }

    println!("{}", ans);
}