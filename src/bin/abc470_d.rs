use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut p: [usize; n],
    }
    for i in 0..n {
        p[i] -= 1;
    }
    let mut p2i: Vec<usize> = vec![0; n];
    for (i, &pi) in p.iter().enumerate() {
        p2i[pi] = i;
    }
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                x: usize,
                y: usize,
            }
            let px = p[x-1];
            let py = p[y-1];
            p.swap(x-1, y-1);
            p2i.swap(px, py);
        } else {
            std::mem::swap(&mut p, &mut p2i);
        }
    }
    for i in 0..n {
        p[i] += 1;
    }
    println!("{}", p.iter().join(" "));
}