use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(usize, char); q],
    }
    let abc = vec!['A', 'B', 'C'];
    let mut cnt = 0;
    for i in 0..(n-2) {
        if s[i..(i+3)] == abc {
            cnt += 1;
        }
    }
    for i in 0..q {
        let (mut x, c) = xc[i];
        x -= 1;
        let start = if x > 1 { x-2 } else { 0 };
        let end = x.min(n-3);
        for j in start..=end {
            if s[j..(j+3)] == abc {
                cnt -= 1;
                break;
            }
        }
        s[x] = c;
        for j in start..=end {
            if s[j..(j+3)] == abc {
                cnt += 1;
                break;
            }
        }
        println!("{}", cnt);
    }
}