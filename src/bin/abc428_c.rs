use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut vec: Vec<(isize, isize, isize)> = Vec::new();  // 1. '('の数, 2. ')'の数, 1の数-2の数の最小値
    let (mut v1, mut v2, mut v3) = (0, 0, 0);
    vec.push((v1, v2, v3));
    for _ in 0..q {
        input! {
            x: usize,
        }
        if x == 1 {
            input! {
                c: char,
            }
            if c == '(' {
                v1 += 1;
            } else {
                v2 += 1;
            }
            v3 = v3.min(v1-v2);
            vec.push((v1, v2, v3));
        } else {
            vec.pop();
            (v1, v2, v3) = *vec.last().unwrap();
        }
        if v1-v2 == 0 && v3 >= 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}