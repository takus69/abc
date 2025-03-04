use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    }
    if a[0]*a[1]==a[2] || a[1]*a[2]==a[0] || a[2]*a[0]==a[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}