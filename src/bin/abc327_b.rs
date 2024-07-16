use proconio::input;

fn main() {
    input! {
        b: usize,
    }
    let mut ans = -1;
    for i in 1..=16 {
        if (i as usize).pow(i as u32) > b { break; }
        if b == (i as usize).pow(i as u32) {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}