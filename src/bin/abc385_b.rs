use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: usize,
        mut y: usize,
        s: [Chars; h],
        t: Chars,
    }
    x -= 1;
    y -= 1;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut c = 0;
    if s[x][y] == '@' {
        visited.insert((x, y));
        c += 1;
    }
    for &ti in t.iter() {
        match ti {
            'U' => { if s[x-1][y] != '#' { x -= 1; } },
            'D' => { if s[x+1][y] != '#' { x += 1; } },
            'L' => { if s[x][y-1] != '#' { y -= 1; } },
            'R' => { if s[x][y+1] != '#' { y += 1; } },
            _ => {},
        }
        if s[x][y] == '@' && !visited.contains(&(x, y)) {
            c += 1;
            visited.insert((x, y));
        }
    }
    println!("{} {} {}", x+1, y+1, c);
}