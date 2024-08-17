use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut bra: Vec<usize> = Vec::new();  // index
    let mut del: Vec<(usize, usize)> = Vec::new();  // (start, end)
    for i in 0..n {
        if s[i] == '(' {
            bra.push(i);
        } else if s[i] == ')' && !bra.is_empty() {
            let start = bra.pop().unwrap();
            del.push((start, i));
        }
    }
    let mut ans: Vec<char> = Vec::new();
    let mut i = n;
    while i > 0 {
        let mut start = 0;
        let mut end = 0;
        if !del.is_empty() {
            (start, end) = del.pop().unwrap();
        }
        if i < start { continue; }
        // println!("i: {}, start: {}, end: {}", i, start, end);
        let mut tmp = if end != 0 { s[(end+1)..i].to_vec() } else { s[0..i].to_vec() };
        tmp.reverse();
        ans.extend(tmp);
        i = start;
    }
    ans.reverse();
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}