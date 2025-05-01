use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let ab = ["AB", "BC", "CD", "DE", "EA", "AE", "ED", "DC", "CB", "BA"];
    if !(ab.contains(&&s[0..]) ^ ab.contains(&&t[0..])) {
        println!("Yes");
    } else {
        println!("No");
    }
}