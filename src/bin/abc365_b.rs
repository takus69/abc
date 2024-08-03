use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut a2: Vec<(usize, usize)> = Vec::new();
    for (i, ai) in a.iter().enumerate() {
        a2.push((*ai, i+1));
    }
    a2.sort();
    println!("{}", a2[n-2].1);
}