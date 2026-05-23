use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }
    for i in 0..(n-1) {
        for j in (i+1)..n {
            if st[i].0 == st[j].0 && st[i].1 == st[j].1 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}