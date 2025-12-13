use proconio::input;

fn main() {
    input! {
        n: usize,
        p :[usize; n],
    }
    let mut ans: usize = 0;
    let mut visited: Vec<bool> = vec![false; n];
    for i in 0..n {
        let mut cnt = 0;
        if visited[i] { continue; }
        let mut pi = i;
        while !visited[pi] {
            visited[pi] = true;
            pi = p[pi]-1;
            cnt += 1;
        }
        ans += cnt*(cnt-1)/2
    }
    println!("{}", ans);
}