use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let l1 = a*a + b*b;
    let l2 = b*b + c*c;
    let l3 = c*c + a*a;
    if l1 == l2 || l2 == l3 || l3 == l1 {
        println!("Yes");
    } else {
        println!("No");
    }
}