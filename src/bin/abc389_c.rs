use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut ans = vec![0; q+1];
    let mut head = 0;
    let mut tail = 0;
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                l: usize,
            }
            let s = ans[tail];
            tail += 1;
            ans[tail] = s + l;
        } else if c == 2 {
            head += 1;
        } else {
            input! {
                k: usize,
            }
            println!("{}", ans[head + k - 1]-ans[head]);
        }
    }
}