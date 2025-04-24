use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let mut ans: Vec<(usize, usize)> = Vec::new();
    let mut now = l;
    while now < r {
        let mut opt_w = 1;
        let mut opt_base = now;
        for i in 0..=60 {
            let w = 2_usize.pow(i);
            if now % w != 0 { continue; }
            let base = now / w;
            if (base+1) * w > r { continue; }
            opt_w = w;
            opt_base = base;
        }
        let next = (opt_base+1) * opt_w;
        ans.push((now, next));
        now = next;
    }

    println!("{}", ans.len());
    for (a, b) in ans.iter() {
        println!("{} {}", a, b);
    }
}