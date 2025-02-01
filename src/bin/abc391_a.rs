use proconio::{input, marker::Chars};

fn main() {
    input! {
        d: Chars,
    }
    let mut ans = Vec::new();
    for di in d.iter() {
        let ai = match di {
            'N' => { 'S' },
            'S' => { 'N' },
            'W' => { 'E' },
            'E' => { 'W' },
            _ => { 'X' },
        };
        ans.push(ai);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}