use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut a2: Vec<usize> = Vec::new();
    let mut a3: Vec<usize> = Vec::new();
    let mut pre_a = 0;
    for &ai in a.iter() {
        if pre_a == ai {
            a3.push(ai);
        } else {
            a2.push(ai);
        }
        pre_a = ai;
    }
    a2.extend(a3);
    let a = a2;
    let mut ans = 0;
    let mut l = 0;
    let mut r = n;
    while l < r {
        if a[l] == ans+1 {
            l += 1;
            ans += 1;
        } else if r-l > 1 {
            r -= 2;
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}