use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            k: usize,
        }

        fn check(x: usize) -> bool {
            let s = x.to_string().chars().collect::<Vec<char>>();
            let mut pre_si = ' ';
            for &si in &s {
                if pre_si == '0' && si == '0' {
                    return true;
                }
                pre_si = si;
            }
            false
        }

        let mut ans = k;
        while !check(ans) {
            ans += k;
        }
        println!("{}", ans);
    }
}