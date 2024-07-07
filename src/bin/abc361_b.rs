use proconio::input;

fn main() {
    input! {
        c1: [usize; 3],
        c2: [usize; 3],
        d1: [usize; 3],
        d2: [usize; 3],
    }
    let mut ans = "No";
    if c1[0].max(d1[0]) < c2[0].min(d2[0]) &&  c1[1].max(d1[1]) < c2[1].min(d2[1]) && c1[2].max(d1[2]) < c2[2].min(d2[2]) { ans = "Yes"; }
    println!("{}", ans);
}