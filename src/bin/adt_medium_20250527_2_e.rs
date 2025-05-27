use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; m],
        a: [usize; m],
    }
    if a.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }
    let mut xa: Vec<(usize, usize)> = Vec::new();
    for i in 0..m {
        xa.push((x[i], a[i]));
    }
    xa.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ans = 0;
    let mut now = 0;
    for i in 0..m {
        let (xi, ai) = xa[i];
        if now+1 < xi {
            println!("-1");
            return;
        }
        let len = now+ai-xi + now+1-xi;
        ans += ai*len/2;
        now += ai;
    }
    println!("{}", ans);
}