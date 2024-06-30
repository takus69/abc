use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let r#mod = 1000000007;
    let mut ans = 1;
    let mut flg = false;
    let mut cnt: i64 = 0;
    let mut i = 0;
    let mut c = "";

    while n >= 2 && i <= n-2 {
        // println!("s {} {}", i, cnt);
        if flg && &s[i..(i+2)] == c {
            cnt += 1;
            i += 2;
            // println!("c {} {}", i, cnt);
            continue;
        } else {
            ans *= cnt + 1;
            ans %= r#mod;
            flg = false;
        }
        if !flg {
            cnt = 0;
            flg = true;
            if &s[i..(i+1)] == "A" {
                c = "BA";
            } else {
                c = "AB";
            }
        }
        i += 1;
    }
    ans *= cnt + 1;
    ans %= r#mod;

    println!("{}", ans);
}