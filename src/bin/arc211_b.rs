use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    }
    let mut s1: Vec<usize> = Vec::new();
    let mut s2: Vec<usize> = Vec::new();
    let mut s3: Vec<usize> = Vec::new();

    if x == y {
        for _ in 0..x {
            s1.push(0);
            s2.push(0);
            s3.push(0);
        }
        if z > y {
            for _ in 0..(z-y) {
                s2.push(0);
                s3.push(0);
            }
        }
    } else {
        for _ in 0..(x-1) {
            s1.push(0);
            s2.push(0);
        }

        for _ in 0..y {
            s1.push(1);
            s3.push(1);
        }

        for i in 0..z {
            s2.push((i+1)%2);
            if i > 0 {
                s3.push((i+1)%2);
            }
        }
    }

    println!("{} {}", s1.len(), s1.iter().join(" "));
    println!("{} {}", s2.len(), s2.iter().join(" "));
    println!("{} {}", s3.len(), s3.iter().join(" "));
}