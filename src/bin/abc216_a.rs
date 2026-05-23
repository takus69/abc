use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let s: Vec<&str> = s.split(".").collect();
    let x: usize = s[0].parse().unwrap();
    let y: usize = s[1].parse().unwrap();
    match y {
        0..=2 => {
            println!("{}-", x);
        },
        3..=6 => {
            println!("{}", x);
        },
        _ => {
            println!("{}+", x);
        }
    }
}