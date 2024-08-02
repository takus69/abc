use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut max_p = 0;
    if n > 1 {
        max_p = *p[1..n].iter().max().unwrap();
    }
    if p[0] <= max_p {
        println!("{}", max_p-p[0]+1);
    } else {
        println!("0");
    }
}