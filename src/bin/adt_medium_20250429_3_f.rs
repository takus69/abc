use proconio::input;

fn main() {
    input! {
        mut sx: usize,
        sy: usize,
        mut tx: usize,
        ty: usize,
    }
    let mut ans = sy.abs_diff(ty);
    if (sy%2 == 0 && sx%2 == 0) || (sy%2 == 1 && sx%2 == 1) { sx += 1; }
    if (ty%2 == 0 && tx%2 == 0) || (ty%2 == 1 && tx%2 == 1) { tx += 1; }
    let mut x_diff = sx.abs_diff(tx);
    if x_diff > ans { x_diff -= ans; } else { x_diff = 0; }
    x_diff /= 2;
    ans += x_diff;
    println!("{}", ans);
}