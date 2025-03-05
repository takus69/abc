use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }
    let mut i = 0;
    let mut j = 0;
    let mut visited = vec![vec![false; w]; h];
    while !visited[i][j] {
        let d = g[i][j];
        visited[i][j] = true;
        match d {
            'U' => {
                if i > 0 {
                    i -= 1;
                } else {
                    println!("{} {}", i+1, j+1);
                    std::process::exit(0);
                }
            },
            'D' => {
                if i < h-1 {
                    i += 1;
                } else {
                    println!("{} {}", i+1, j+1);
                    std::process::exit(0);
                }
            },
            'L' => {
                if j > 0 {
                    j -= 1;
                } else {
                    println!("{} {}", i+1, j+1);
                    std::process::exit(0);
                }
            },
            'R' => {
                if j < w-1 {
                    j += 1;
                } else {
                    println!("{} {}", i+1, j+1);
                    std::process::exit(0);
                }
            },
            _ => {},
        }
    }
    println!("-1");
}