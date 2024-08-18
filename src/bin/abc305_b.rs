use proconio::input;

fn main() {
    input! {
        p: char,
        q: char,
    }
    let str = "ABCDEFG";
    let mut pi = str.chars().position(|c| c == p).unwrap();
    let mut qi = str.chars().position(|c| c == q).unwrap();
    let dist = vec![3, 1, 4, 1, 5, 9];
    // println!("pi: {}, qi: {}", pi, qi);
    if pi > qi {
        std::mem::swap(&mut pi, &mut qi);
    }
    let mut ans = 0;
    for i in pi..qi {
        ans += dist[i];
    }
    println!("{}", ans);
}