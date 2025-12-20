use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
        b: [usize; n],
    }
    let mut ans: Vec<usize> = vec![0; h];
    for bi in b.iter() {
        for i in 0..h {
            if a[i].contains(bi) {
                ans[i] += 1;
                break;
            }
        }
    }
    println!("{}", ans.iter().max().unwrap());
}