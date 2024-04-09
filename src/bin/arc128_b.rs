use proconio::input;
            
fn func(r: i32, g: i32) -> i32 {
    let mut ans = (r - g).abs();
    ans += r.min(g);
    // println!("r: {} g: {} ans: {}", r, g, ans);
    ans
}

fn main() {
    input! {
        t: i32,
        cases: [(i32, i32, i32); t],
    };
    for (r, g, b) in cases {
        let mut ans = std::i32::MAX;
        if (r - g) % 3 == 0 {
            ans = ans.min(func(r, g));
        }
        if (g - b) % 3 == 0 {
            ans = ans.min(func(g, b));
        }
        if (b - r) % 3 == 0 {
            ans = ans.min(func(b, r));
        }
        if ans == std::i32::MAX {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}