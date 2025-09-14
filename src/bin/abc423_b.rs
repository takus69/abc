use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n],
    }
    let mut li = 0;
    let mut ri = n;
    for i in 0..n {
        if l[i] == 1 {
            break;
        } else {
            li = i+1;
        }
    }
    for i in (0..n).rev() {
        if l[i] == 1 {
            break;
        } else {
            ri = i;
        }
    }
    if li >= ri {
        println!("0");
    } else {
        println!("{}", ri-li-1);
    }
}