use proconio::{input, marker::Chars};
use ac_library::Dsu;
use std::collections::HashSet;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let mut dsu = Dsu::new(h*w);
    fn index(i: usize, j: usize, w: usize) -> usize {
        i*w + j
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' { continue; }
            for (di, dj) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let i2 = i as isize + di;
                let j2 = j as isize + dj;
                if i2 < 0 || i2 >= h as isize || j2 < 0 || j2 >= w as isize { continue; }
                let i2 = i2 as usize;
                let j2 = j2 as usize;
                if s[i2][j2] == '.' {
                    dsu.merge(index(i, j, w), index(i2, j2, w));
                    // println!("merge: ({}, {})-({}, {})", i, j, i2, j2);
                }
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            dsu.leader(index(i, j, w));
        }
    }
    let mut set: HashSet<usize> = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' { continue; }
            let l = dsu.leader(index(i, j, w));
            set.insert(l);
        }
    }
    let mut set2: HashSet<usize> = HashSet::new();
    for i in 0..h {
        if s[i][0] == '.' {
            let l = dsu.leader(index(i, 0, w));
            set2.insert(l);
        }
        if s[i][w-1] == '.' {
            let l = dsu.leader(index(i, w-1, w));
            set2.insert(l);
        }
    }
    for j in 0..w {
        if s[0][j] == '.' {
            let l = dsu.leader(index(0, j, w));
            set2.insert(l);
        }
        if s[h-1][j] == '.' {
            let l = dsu.leader(index(h-1, j, w));
            set2.insert(l);
        }
    }
    // println!("set: {:?}, set2: {:?}", set, set2);
    println!("{}", set.len()-set2.len());
}