use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut pre = a[0]-1;
    for ai in a.iter() {
        if *ai == pre+1 {
            pre = *ai
        } else {
            break
        }
    }
    println!("{}", pre+1);
}