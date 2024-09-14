use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    }
    if (s[0] == "<" && s[1] == ">") || (s[0] == ">" && s[1] == "<") {
        println!("A");
    } else if (s[0] == "<" && s[2] == "<") || (s[0] == ">" && s[2] == ">") {
        println!("B");
    } else {
        println!("C");
    }
}