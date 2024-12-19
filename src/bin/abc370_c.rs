use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    let mut ans: Vec<String> = Vec::new();

    // 前から順に辞書順が小さくなるなら置換する
    for i in 0..s.len() {
        if s[i] > t[i] {
            s[i] = t[i];
            ans.push(s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
        }
    }

    // 後ろから順に置換する(辞書順が大きくなるパターン)
    for i in (0..s.len()).rev() {
        if s[i] != t[i] {
            s[i] = t[i];
            ans.push(s.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
        }
    }

    println!("{}", ans.len());
    for a in ans.iter() {
        println!("{}", a);
    }
}