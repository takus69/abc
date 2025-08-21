use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    for i in 0..h {
        for j in 0..(w-1) {
            x[i][j+1] += x[i][j];
        }
    }
    for j in 0..w {
        for i in 0..(h-1) {
            x[i+1][j] += x[i][j];
        }
    }
    for &(a, b, c, d) in abcd.iter() {
        println!("{}", x[c-1][d-1]- if b==1 { 0 } else { x[c-1][b-2] } - if a==1 { 0 } else { x[a-2][d-1] } + if a==1 || b==1 { 0 } else { x[a-2][b-2]});
    }
}