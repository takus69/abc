use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            p: [usize; n],
        }
        let mut ans = 2;
        if p[0] == n && p[n-1] == 1 {
            ans = 3;
        }
        let mut max_l = 0;
        let mut flg = true;
        for (i, pi) in p.iter().enumerate() {
            if i+1 == *pi && max_l < i+1 {
                // println!("i: {}, pi: {}", i, pi);
                ans = 1;
            } else {
                flg = false;
            }
            max_l = max_l.max(*pi);
        }
        if flg {
            ans = 0;
        }
        println!("{}", ans);
    }
}