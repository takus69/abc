use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            h: usize,
            tlu: [(usize, usize, usize); n],
        }

        let mut ans = "Yes";
        let mut now_u = h;
        let mut now_l = h;
        let mut now_t = 0;
        for &(ti, li, ui) in tlu.iter() {
            let d = ti - now_t;
            let next_l = now_l.saturating_sub(d);
            let next_u = now_u + d;
            if next_l > ui || next_u < li {
                ans = "No";
                break;
            }
            
            now_t = ti;
            now_u = next_u.min(ui);
            now_l = next_l.max(li);
        }

        println!("{}", ans);
    }
}