use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.sort();
    b.sort();
    let mut ans = 0;
    while let Some(bi) = b.pop() {
        if a.is_empty() { break; }
        let ai = a.last().unwrap();
        if ai*2 >= bi {
            ans += 1;
            a.pop();
        }
    }
    println!("{}", ans);
}