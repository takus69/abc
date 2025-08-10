use proconio::input;
use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }    
}

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    let mut map2: HashMap<(isize, isize, isize), usize> = HashMap::new();
    for i in 0..n {
        for j in (i+1)..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let mut dx = x2-x1;
            let mut dy = y2-y1;
            let g = gcd(dx.abs() as usize, dy.abs() as usize) as isize;
            dx /= g;
            dy /= g;
            if dx < 0 {
                dx *= -1;
                dy *= -1;
            } else if dx == 0 && dy < 0 {
                dy *= -1;
            }
            let e = map.entry((dx, dy)).or_insert(0);
            *e += 1;

            let e = map2.entry((dx, dy, (x2-x1)*(x2-x1)+(y2-y1)*(y2-y1))).or_insert(0);
            *e += 1;
        }
    }
    let mut ans = 0;
    for (_, &cnt) in map.iter() {
        ans += cnt*(cnt-1)/2;
    }
    let mut ans2 = 0;
    for (_, &cnt) in map2.iter() {
        ans2 += cnt*(cnt-1)/2;
    }
    ans -= ans2/2;
    println!("{}", ans);
}