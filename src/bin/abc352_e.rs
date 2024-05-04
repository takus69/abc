use proconio::input;
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut kca = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            c: usize,
            a: [usize; k],
        }
        kca.push((k, c, a));
    }
    kca.sort_by(|a, b| a.1.cmp(&b.1));
    
    let mut dsu = Dsu::new(n+1);
    let mut ans = 0;
    for (k, c, a) in kca {
        for i in 0..(k-1) {
            if !dsu.same(a[i], a[i+1]) {
                dsu.merge(a[i], a[i+1]);
                ans += c;
            }
        }
    }
    let mut flg = true;
    for i in 2..=n {
        if !dsu.same(1, i) {
            flg = false;
            break;
        }
    }
    if flg {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}