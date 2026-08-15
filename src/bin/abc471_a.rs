use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }
    if a+b==9 || a-b==9 || a*b==9 || (a/b==9&&a%b==0) {
        println!("Nine");
    } else {
        println!("Nein");
    }
}