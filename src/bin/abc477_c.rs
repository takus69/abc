use proconio::{input, marker::Chars};

fn main() {
    input! {
        q: usize,
        s: Chars,
        t: Chars,
    }
    let mut pos: Vec<usize> = Vec::new();
    let n = s.len();
    let len = t.len();
    if n < len {
        for _ in 0..q {
            println!("No");
        }
        return;
    }
    for i in 0..(s.len()-len+1) {
        if s[i..(i+len)] == t {
            pos.push(i);
        }
    }
    for _ in 0..q {
        input! {
            mut l: usize,
            mut r: usize,
        }
        l -= 1;
        r -= 1;

        let i = pos.partition_point(|&x| x < l);
        // println!("l: {}, r: {}, i: {}, pos: {:?}", l, r, i, pos);
        if i < pos.len() && pos[i] < n && pos[i]+len-1 <= r {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}