use proconio::input;

fn main() {
    input! {
        mut s: String,
    }
    let l = s.pop().unwrap();
    if l == 'e' {
        s.push(l);
        s.push('r');
    } else {
        s.push(l);
        s.push('e');
        s.push('r');
    }
    println!("{}", s);
}