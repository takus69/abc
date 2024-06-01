use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    }

    sc.sort();

    let mut t = 0;
    for (_, c) in sc.iter() {
        t += c;
    }

    println!("{}", sc[t%n].0);
}