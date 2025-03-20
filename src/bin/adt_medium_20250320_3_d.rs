use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k],
    }
    let max_a = a.iter().max().unwrap();
    let mut ans: Vec<usize> = Vec::new();
    for (i, ai) in a.iter().enumerate() {
        if max_a == ai {
            ans.push(i+1);
        }
    }
    for bi in b.iter() {
        if ans.contains(bi) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}