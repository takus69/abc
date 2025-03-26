use proconio::input;

fn main() {
    input! {
        sx: usize,
        sy: usize,
        tx: usize,
        ty: usize,
    }
    let y_diff = sy.abs_diff(ty);
    let x_diff = sx.abs_diff(tx);
    let mut ans = y_diff;
    if x_diff > y_diff {
        if sx < tx  {
            if (sx+sy)%2 == 0 {
                ans += (x_diff - y_diff - 1 + 1)/2;
            } else {
                ans += (x_diff - y_diff + 1)/2;
            }
        } else {
            if (sx+sy)%2 == 0 {
                ans += (x_diff - y_diff + 1)/2;
            } else {
                ans += (x_diff - y_diff - 1 + 1)/2;
            }
        }
    }
    println!("{}", ans);
}