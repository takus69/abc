use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let up_cnt = s.chars().filter(|x| x.is_uppercase()).count();
    if s.len() - up_cnt < up_cnt {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}