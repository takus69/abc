use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut cnt: Vec<usize> = vec![0; k];
    for &ai in &a {
        cnt[ai-1] += 1;
    }
    cnt.sort();
    let mut ans = 1;
    let now_cnt = cnt.pop().unwrap();
    while let Some(c) = cnt.pop() {
        if now_cnt-1 <= c {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}