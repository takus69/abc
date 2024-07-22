use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize; n],
    }
    let mut cnt = 0;
    let mut ans = 0;
    l.sort();
    for li in l.iter().rev() {
        if cnt >= p {
            break;
        }
        if li >= &t {
            cnt += 1;
        } else {
            cnt += 1;
            ans = ans.max(t-li);
        }
    }
    println!("{}", ans);
}