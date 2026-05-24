use proconio::input;

fn main() {
    input! {
        s: [String; 3],
    }
     for si in ["ABC", "ARC", "AGC", "AHC"] {
        // if !s.contains(&(si.to_string())) {
        if !s.iter().any(|x| x == si) {
            println!("{}", si);
        }
    }
}