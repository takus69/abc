use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        x: usize,
        y: usize,
        abc: [usize; 3],
    }
    // 3つ並べるパターン1
    fn pattern1(x: usize, y: usize, abc: &[usize]) -> bool {
        let (a, b, c) = (abc[0], abc[1], abc[2]);
        let mut needed: usize = 0;
        needed += (a-1)/x + 1;
        needed += (b-1)/x + 1;
        needed += (c-1)/x + 1;
        
        needed <= y
    }
    // 1-2で並べるパターン2
    fn pattern2(x: usize, y: usize, abc: &[usize]) -> bool {
        let (a, b, c) = (abc[0], abc[1], abc[2]);
        let dy = (a-1)/x + 1;
        let y2 = if y > dy {
            y-dy
        } else {
            return false;
        };
        let mut needed = (b-1)/y2 + 1;
        needed += (c-1)/y2 + 1;
        
        needed <= x
    }

    let mut ans = false;
    if pattern1(x, y, &abc) { ans = true; } 
    if pattern1(y, x, &abc) { ans = true; } 
    for perm in abc.into_iter().permutations(3) {
        if pattern2(x, y, &perm) { ans = true; }
        if pattern2(y, x, &perm) { ans = true; }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}