use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }
    let mut pre = 0;
    let mut ans = "Yes";
    for si in s.iter() {
        if !(pre <= *si && *si >= 100 && *si <= 675 && si%25 == 0) {
            ans = "No";
        }
        pre = *si;
    }
    println!("{}", ans);
}