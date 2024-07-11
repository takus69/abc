use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    let mut x = 0;
    let mut y = 0;
    for (xi, yi) in xy.iter() {
        x += xi;
        y += yi;
    }
    if x == y {
        println!("Draw");
    } else if x > y {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}