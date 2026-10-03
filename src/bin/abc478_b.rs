use proconio::input;

fn main() {
    input! {
        n: usize,
        v: usize,
        w: [usize; n],
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            for k in 0..n {
                if i == k || j == k { continue; }
                let cost = i+j+k+3;
                let w2 = w[i] + w[j] + w[k];
                if cost <= v {
                    ans = ans.max(w2);
                }
            }
        }
    }
    println!("{}", ans);
}