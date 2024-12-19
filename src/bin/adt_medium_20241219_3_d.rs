use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut x: Vec<usize> = vec![0; w];
    for j in 0..w {
        for i in 0..h {
            if c[i][j] == '#' {
                x[j] += 1;
            }
        }
    }
    println!("{}", x.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}