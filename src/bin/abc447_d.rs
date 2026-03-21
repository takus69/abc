use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let (mut a_cnt, mut b_cnt, mut c_cnt) = (0, 0, 0);

    for &si in s.iter() {
        match si {
            'A' => {
                a_cnt += 1;
            },
            'B' => {
                if a_cnt > b_cnt {
                    b_cnt += 1;
                }
            },
            'C' => {
                if b_cnt > c_cnt {
                    c_cnt += 1;
                }
            },
            _ => {}
        }
    }

    println!("{}", c_cnt);
}