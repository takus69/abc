use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    fn check(x1: isize, y1: isize, x2: isize, y2: isize, x3: isize, y3: isize) -> bool {
        let v2 = (x2-x1, y2-y1);
        let v3 = (x3-x1, y3-y1);
        v2.0*v3.1 - v2.1*v3.0 != 0
    }

    let mut ans = 0;
    for i1 in 0..(n-2) {
        for i2 in (i1+1)..(n-1) {
            for i3 in (i2+1)..n {
                let (x1, y1) = xy[i1];
                let (x2, y2) = xy[i2];
                let (x3, y3) = xy[i3];
                if check(x1, y1, x2, y2, x3, y3) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}