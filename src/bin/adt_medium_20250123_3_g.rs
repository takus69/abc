use proconio::{input, input_interactive};

fn main() {
    input_interactive! {
        n: usize,
    }
    let mut ok = 1;
    let mut ng = n;
    while ok + 1 < ng {
        let m = (ok + ng) / 2;
        println!("? {}", m);

        input_interactive! {
            p: usize,
        }
        if p == 0 {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("! {}", ok);
}