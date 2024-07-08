use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        t: [usize; n],
    }
    let mut pre = 0;
    for ti in t.iter() {
        if &pre > ti {
            pre += a;
        } else {
            pre = ti + a;
        }
        println!("{}", pre);
    }
}