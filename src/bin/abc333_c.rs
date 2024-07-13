use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut base: Vec<usize> = vec![1];
    for _ in 0..11 {
        let mut tmp = base[base.len()-1];
        tmp *= 10;
        tmp += 1;
        base.push(tmp);
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..base.len() {
        for j in i..base.len() {
            for k in j..base.len() {
                ans.push(base[i] + base[j] + base[k]);
            }
        }
    }
    ans.sort();
    println!("{}", ans[n-1]);
}