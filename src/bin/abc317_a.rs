use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        p: [usize; n],
    }
    for (i, pi) in p.iter().enumerate() {
        if h + pi >= x {
            println!("{}", i+1);
            break;
        }
    }
}