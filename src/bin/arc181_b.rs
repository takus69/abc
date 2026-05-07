use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            s: Chars,
            x: Chars,
            y: Chars,
        }
        // 0の数が同じなら達成可能
        // 0の数が違う場合は、0の数を1の数で表現できるか
        // 0が多い => 0を繋げて、1の数ごとに繰返しか
        let m = s.len();
        let mut x0 = 0;
        let mut x1 = 0;
        let mut y0 = 0;
        let mut y1 = 0;
        for xi in x.iter() {
            if xi == &'0' {
                x0 += 1;
            } else {
                x1 += 1;
            }
        }
        for yi in y.iter() {
            if yi == &'0' {
                y0 += 1;
            } else {
                y1 += 1;
            }
        }

        // m*(x0-y0)=k*(y1-x1)
        // println!("x0: {}, x1: {}, y0: {}, y1: {}", x0, x1, y0, y1);
        if x0==y0 { yes(); continue; } else if x1==y1 { no(); continue; }
        let (z0, z1) = if x0 > y0 {
            if x1 > y1 { no();continue; }
            (x0-y0, y1-x1)
        } else {
            if x1 < y1 { no();continue; }
            (y0-x0, x1-y1)
        };
        // println!("z0: {}, z1: {}", z0, z1);
        let k = if (m*z0)%z1==0 { (m*z0)/z1 } else { no(); continue; };
        let g = gcd(k, m);
        let mut flg = true;
        // println!("m: {}, k: {}, g: {}", m, k, g);
        for i in 0..m {
            if s[i] != s[(i+g)%m] { flg = false;break; }
        }
        if flg { yes(); } else { no(); }
    }

    fn no() { println!("No"); }
    fn yes() { println!("Yes"); }
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a%b) }
    }
}