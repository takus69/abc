use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: usize,
        mut y: usize,
        mut s: [Chars; h],
        t: Chars,
    }
    (x, y) = (x-1, y-1);
    let mut ans = 0;
    for &ti in t.iter() {
        let (x2, y2) = match ti {
            'U' => { (x-1, y) },
            'D' => { (x+1, y) },
            'L' => { (x, y-1) },
            'R' => { (x, y+1) },
            _ => { (x, y) },
        };
        if s[x2][y2] == '#' { continue; }
        if s[x2][y2] == '@' {
            ans += 1;
            s[x2][y2] = '.';
        }
        (x, y) = (x2, y2);
    }
    println!("{} {} {}", x+1, y+1, ans);
}