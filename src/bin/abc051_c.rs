use proconio::input;

fn main() {
    input! {
        sx: i64,
        sy: i64,
        tx: i64,
        ty: i64,
    }
    let mut ans = "".to_string();
    let dirs1 = vec!["R".to_string(), "U".to_string()];
    let dirs2 = vec!["L".to_string(), "D".to_string()];
    // 1回目
    for _ in 0..(tx-sx) {
        ans += &dirs1[0];
    }
    for _ in 0..(ty-sy) {
        ans += &dirs1[1];
    }
    for _ in 0..(tx-sx) {
        ans += &dirs2[0];
    }
    for _ in 0..(ty-sy) {
        ans += &dirs2[1];
    }

    // 2回目
    ans += &dirs2[1];
    for _ in 0..(tx-sx+1) {
        ans += &dirs1[0];
    }
    for _ in 0..(ty-sy+1) {
        ans += &dirs1[1];
    }
    ans += &dirs2[0];
    ans += &dirs1[1];
    for _ in 0..(tx-sx+1) {
        ans += &dirs2[0];
    }
    for _ in 0..(ty-sy+1) {
        ans += &dirs2[1];
    }
    ans += &dirs1[0];

    println!("{}", ans);
}
