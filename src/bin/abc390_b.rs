use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    if n == 2 {
        println!("Yes");
        std::process::exit(0);
    }
    for i in 1..(n-1) {
        if a[i-1]*a[i+1] != a[i]*a[i] {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}