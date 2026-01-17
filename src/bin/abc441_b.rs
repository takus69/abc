use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        w: [Chars; q],
    }
    for wi in w.iter() {
        let mut t_flag = true;
        let mut a_flag = true;
        for wii in wi.iter() {
            if !s.contains(wii) {
                t_flag = false;
                break;
            }
        }
        for wii in wi.iter() {
            if !t.contains(wii) {
                a_flag = false;
                break;
            }
        }
        if (t_flag && a_flag) || (!t_flag && !a_flag) {
            println!("Unknown");
        } else if t_flag {
            println!("Takahashi ");
        } else {
            println!("Aoki ");
        }
    }
}