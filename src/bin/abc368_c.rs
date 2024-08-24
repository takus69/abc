use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let mut t = 0;
    for hi in h.iter() {
        let ti = (t+1)%3;
        // println!("t: {}, ti: {}", t, ti);
        t += 3*(((hi-1) / 5) + 1);
        if hi % 5 == 1 {
            t-= 2;
        } else if hi % 5 == 2 {
            if ti == 0 {
                t -= 2;
            } else {
                t -= 1;
            }
        } else if hi % 5 == 3 {
            if ti == 0 {
                t -= 2;
            } else if ti == 2 {
                t -= 1
            }
        } else if hi % 5 == 4 {
            if ti == 0 || ti == 2{
                t -= 1;
            }
        }
        // println!("t: {}", t);
    }
    println!("{}", t);
}