use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        l: usize,
        r: usize,
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    if a <= l {
        if b <= l {
            ans += y*(b-a);
        } else {
            ans += y*(l-a);
            if b <= r {
                ans += x*(b-l);
            } else {
                ans += x*(r-l)+y*(b-r);
            }
        }
    } else if a <= r {
        if b <= r {
            ans += x*(b-a);
        } else {
            ans += x*(r-a)+y*(b-r);
        }
    } else {
        ans += y*(b-a);
    }

    println!("{}", ans);
}