use proconio::{input, marker::Chars};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    fn rotate(n: usize, x: usize, y: usize, i: usize) -> (usize, usize) {
        match i%4 {
            0 => {
                (y, n-x-1)
            },
            1 => {
                (n-x-1, n-y-1)
            },
            2 => {
                (n-y-1, x)
            },
            _ => { (x, y) },
        }
    }

    let mut b = a.clone();
    for i in 0..(n/2) {
        for x in i..(n-i) {
            let y = i;
            let (x2, y2) = rotate(n, x, y, i);
            b[x2][y2] = a[x][y];
            let y = n-i-1;
            let (x2, y2) = rotate(n, x, y, i);
            b[x2][y2] = a[x][y];
        }
        for y in i..(n-i) {
            let x = i;
            let (x2, y2) = rotate(n, x, y, i);
            b[x2][y2] = a[x][y];
            let x = n-i-1;
            let (x2, y2) = rotate(n, x, y, i);
            b[x2][y2] = a[x][y];
            
        }
    }
    for bi in b.iter() {
        println!("{}", bi.iter().join(""));
    }
}