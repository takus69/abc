use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let mut ans = 0;
    let w = (c - a).abs();
    let h = (d - b).abs();
    let w4 = w / 4;
    ans += 4 * h * w4;
    // println!("hw: {} from{}, to{}", ans, (a+w4*4), c);
    for j in (a+w4*4)..c {
        let mut j4 = j % 4;
        if j4 < 0 { j4 += 4; }
        let h2 = h / 2;
        // println!("j: {}, j4: {}, h2: {}", j, j4, h2);
        if j4 == 0 || j4 == 1 {
            ans += 3 * h2;
        } else {
            ans += h2;
        }
        // println!("w: {}", ans);
        if (h % 2) == 1 {
            // println!("j4: {}", j4);
            let mut d2 = d % 2;
            if d < 0 { d2 += 2; }
            if d2 == 1 {
                ans += match j4 {
                    0 => 2,
                    1 => 1,
                    2 => 0,
                    3 => 1,
                    _ => 0,
                }
            } else {
                ans += match j4 {
                    0 => 1,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => 0,
                }
            }
        }
        // println!("h: {}", ans);
    }

    println!("{}", ans);
}