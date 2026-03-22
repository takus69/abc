use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            c: usize,
            d: usize,
        }
        let mut ans = 0;
        for dd in 1..=10 {
            let mut l = 10_usize.pow(dd-1);
            l = l.max(1+c);
            let mut r = 10_usize.pow(dd)-1;
            r = r.min(d+c);
            // println!("dd: {}, {} <= x <= {}, {} <= f <= {}", dd, l, r, c*10_usize.pow(dd)+l, c*10_usize.pow(dd)+r);
            if l > r { continue; }
            let ll = (c*10_usize.pow(dd)+l-1).isqrt();
            let rr = (c*10_usize.pow(dd)+r).isqrt();
            ans += rr-ll;
        }
        
        println!("{}", ans);
    }
}