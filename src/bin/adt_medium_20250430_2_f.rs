use proconio::input;

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    }
    let mut target1: Vec<(isize, isize)> = Vec::new();
    let mut target2: Vec<(isize, isize)> = Vec::new();
    for (dx, dy) in [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)] {
        target1.push((x1+dx, y1+dy));
        target2.push((x2+dx, y2+dy));
    }
    let mut ans = "No";
    for t1 in target1.iter() {
        if target2.contains(t1) {
            ans = "Yes";
        }
    }
    
    println!("{}", ans);
}