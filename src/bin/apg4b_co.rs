use proconio::input;

fn main() {
    input! {
        p: u32,
    }
    if p == 1 {
        input! {
            price: u32,
            n: u32,
        }
        println!("{}", price*n);
    } else {
        input! {
            text: String,
            price: u32,
            n: u32,
        }
        println!("{}!", text);
        println!("{}", price*n);
    }
}