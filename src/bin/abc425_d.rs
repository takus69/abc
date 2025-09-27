use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut before = s.clone();
    let mut after = s.clone();
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if before[i][j] == '#' {
                set.insert((i, j));
            }
        }
    }

    let dirs = [(1, 0), (!0, 0), (0, 1), (0, !0)];
    while !set.is_empty() {
        let mut set2: HashSet<(usize, usize)> = HashSet::new();
        for &(i, j) in set.iter() {
            // 隣接する白を処理
            for &(di, dj) in dirs.iter() {
                let (i2, j2) = (i.wrapping_add(di), j.wrapping_add(dj));
                if i2 >= h || j2 >= w { continue; }
                if before[i2][j2] == '#' { continue; }
                let mut cnt = 0;
                for &(di, dj) in dirs.iter() {
                    let (i3, j3) = (i2.wrapping_add(di), j2.wrapping_add(dj));
                    if i3 >= h || j3 >= w { continue; }
                    if before[i3][j3] == '#' { cnt += 1; }
                }
                if cnt == 1 {
                    set2.insert((i2, j2));
                }
            }
        }
        for &(i, j) in set2.iter() {
            before[i][j] = '#';
        }
        set = set2;
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if before[i][j] == '#' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}