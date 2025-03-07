use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }
    s.sort();
    t.sort();
    fn len(s: &Vec<char>) -> usize {
        if s == &['A', 'B'] || s == &['B', 'C'] || s == &['C', 'D'] || s == &['D', 'E'] || s == &['A', 'E'] {
            1
        } else {
            2
        }
    }
    if len(&s) == len(&t) {
        println!("Yes");
    } else {
        println!("No");
    }
}