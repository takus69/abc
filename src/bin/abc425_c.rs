use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],

    }
    let mut b: Vec<usize> = vec![0];
    let mut tmp = 0;
    for &ai in a.iter() {
        tmp += ai;
        b.push(tmp);
    }
    for &ai in a.iter() {
        tmp += ai;
        b.push(tmp);
    }
    let mut index = 0;
    for _ in 0..q {
        input! {
            c: usize,
        }
        match c {
            1 => {
                input! {
                    c: usize,
                }
                index += c;
                index %= n;
            },
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }
                println!("{}", b[r+index]-b[l+index-1]);
            },
            _ => {},
        }
    }
}