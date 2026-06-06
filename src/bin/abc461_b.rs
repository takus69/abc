use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    for (i, &ai) in a.iter().enumerate() {
        if b[ai-1] != i+1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}