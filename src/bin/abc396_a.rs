use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt = 0;
    let mut ans = "No";
    let mut pre_a = 0;
    for &ai in a.iter() {
        if pre_a == ai {
            cnt += 1;
        } else {
            cnt = 1;
        }
        if cnt == 3 {
            ans = "Yes";
        }
        pre_a = ai;
    }

    println!("{}", ans);
}