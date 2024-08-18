use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                let mut cnt = 0;
                for dir in DIRS {
                    if i == 0 && dir.0 < 0 || i == h-1 && dir.0 > 0 { continue; }
                    if j == 0 && dir.1 < 0 || j == w-1 && dir.1 > 0 { continue; }
                    let di = (i as i32 + dir.0) as usize;
                    let dj = (j as i32 + dir.1) as usize;
                    if s[di][dj] == '#' { cnt += 1; }
                }
                if cnt > 1 {
                    println!("{} {}", i+1, j+1);
                    std::process::exit(0);
                }

            }
        }
    }
}