use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        lr: [(usize, usize); n],
    }
    let mut end: Vec<(usize, usize)> = Vec::new();
    let mut start: Vec<(usize, usize)> = Vec::new();
    for (i, &(l, r)) in lr.iter().enumerate() {
        end.push((r, i));
        start.push((l, i));
    }
    end.sort();
    start.sort();start.reverse();
    let mut destroy: Vec<bool> = vec![false; n+1];
    let mut ans = 0;
    for &(r, i) in &end {
        if destroy[i] { continue; }
        // println!("l: {}, i: {}", l, i);
        ans += 1;
        destroy[i] = true;
        while let Some((l, j)) = start.pop() {
            // println!("ll: {}", ll);
            if destroy[j] { continue; }
            if r+d <= l {
                start.push((l, j));
                break;
            }
            destroy[j] = true;
        }
    }
    println!("{}", ans);
}