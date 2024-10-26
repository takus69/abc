use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize, usize); m],
    }
    for i in 0..m {
        ab[i] = (ab[i].0-1, ab[i].1-1);
    }
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for (a, b) in ab.iter() {
        set.insert((*a, *b));
    }

    fn add(set: &mut HashSet<(usize, usize)>, a: usize, b: usize, da: i64, db: i64, n: usize) {
        let a2 = a as i64 + da;
        let b2 = b as i64 + db;
        let n = n as i64;
        if a2 >= 0 && a2 < n && b2 >= 0 && b2 < n {
            set.insert((a2 as usize, b2 as usize));
        }
    }

    for i in 0..m {
        let (a, b) = ab[i];
        add(&mut set, a, b, 2, 1, n);
        add(&mut set, a, b, 1, 2, n);
        add(&mut set, a, b, -1, 2, n);
        add(&mut set, a, b, -2, 1, n);
        add(&mut set, a, b, -2, -1, n);
        add(&mut set, a, b, -1, -2, n);
        add(&mut set, a, b, 1, -2, n);
        add(&mut set, a, b, 2, -1, n);
    }
    println!("{}", n*n-set.len());
}