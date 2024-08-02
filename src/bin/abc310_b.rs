use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut p: Vec<usize> = Vec::new();
    let mut f: Vec<Vec<bool>> = vec![vec![false; m]; n];
    for i in 0..n {
        input! {
            pi: usize,
            ci: usize,
            fi: [usize; ci],
        }
        p.push(pi);
        for ff in fi {
            f[i][ff-1] = true;
        }
    }
    let mut ans = "No";
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if p[i] < p[j] { continue; }
            let mut add = false;
            let mut all = true;
            for k in 0..m {
                if f[i][k] && !f[j][k] { all = false; break; }
                if !f[i][k] && f[j][k] { add = true; }
            }
            if all && (p[i] > p[j] || add) {
                ans = "Yes";
            }
            if ans == "Yes" { break; }
        }
        if ans == "Yes" { break; }
    }
    println!("{}", ans);
}