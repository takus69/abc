use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut si: usize,
        mut sj: usize,
        c: [Chars; h],
        x: Chars,
    }
    si -= 1;
    sj -= 1;
    for xi in x {
        if xi == 'L' && sj > 0 && c[si][sj-1] == '.' {
            sj -= 1;
        } else if xi == 'R' && sj < w-1 && c[si][sj+1] == '.' {
            sj += 1;
        } else if xi == 'U' && si > 0 && c[si-1][sj] == '.' {
            si -= 1;
        } else if xi == 'D' && si < h-1 && c[si+1][sj] == '.' {
            si += 1;
        }
    }
    println!("{} {}", si+1, sj+1);
}