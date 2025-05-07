use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut t: usize,
        a: [usize; n-1],
        xy: [(usize, usize); m],
    }
    let mut now = 1;
    let mut i = 0;
    while t > 0 {
        let (x, y) = if i < xy.len() { xy[i] } else { (0, 0) };
        let ai = a[now-1];
        if t > ai {
            now += 1;
            t -= ai;
        } else {
            println!("No");
            return;
        }
        if x == now {
            t += y;
            i += 1;
        }
        if now == n {
            break;
        }
    }
    if now == n {
        println!("Yes");
    } else {
        println!("No");
    }
}