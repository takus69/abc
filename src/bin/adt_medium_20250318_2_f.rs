use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(usize, char); q],
    }
    let mut ans = 0;
    for i in 0..(n-2) {
        if s[i..(i+3)] == ['A', 'B', 'C'] {
            ans += 1;
        }
    }
    for &(xi, ci) in xc.iter() {
        let l = match s[xi-1] {
            'A' => { xi as isize -1 },
            'B' => { xi as isize -2 },
            'C' => { xi as isize -3 },
            _ => { isize::MAX },
        };
        let l = if l >= 0 && l < n as isize-2 { l as usize } else { usize::MAX };
        if l != usize::MAX {
            if s[l..(l+3)] == ['A', 'B', 'C'] {
                ans -= 1;
            }
        }
        s[xi-1] = ci;
        let l = match s[xi-1] {
            'A' => { xi as isize -1 },
            'B' => { xi as isize -2 },
            'C' => { xi as isize -3 },
            _ => { isize::MAX },
        };
        let l = if l >= 0 && l < n as isize-2 { l as usize } else { usize::MAX };
        if l != usize::MAX {
            if s[l..(l+3)] == ['A', 'B', 'C'] {
                ans += 1;
            }
        }
        println!("{}", ans);
    }

}