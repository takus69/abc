use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    }
    for si in ["ABC", "ARC", "AGC", "AHC"] {
        if s.contains(&si.to_string()) {
            continue;
        } else {
            println!("{}", si);
            return;
        }
    }
}