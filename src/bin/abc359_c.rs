use proconio::input;

fn main() {
    input! {
        mut sx: i64,
        mut sy: i64,
        mut tx: i64,
        mut ty: i64,
    }

    if sx > tx {
        std::mem::swap(&mut sx, &mut tx);
        std::mem::swap(&mut sy, &mut ty);
    }
    // println!("s: ({}, {}), t: ({}, {})", sx, sy, tx, ty);

    let dy = (ty - sy).abs();
    if (sx + sy) % 2 == 1 {
        sx -= 1;
    }
    if (tx + ty) % 2 == 1 {
        tx -= 1;
    }
    // println!("s: ({}, {}), t: ({}, {})", sx, sy, tx, ty);
    if (sx + dy) < tx { sx += dy; } else { sx = tx; }
    // println!("s: ({}, {}), t: ({}, {})", sx, sy, tx, ty);
    let dx = (tx - sx).abs() / 2;
    let ans = dx + dy;
    // println!("{} {} {} {} {}", sx+sy, tx+ty, ans, (tx+ty)%2, (sx+sy)%2);
    println!("{}", ans);
}