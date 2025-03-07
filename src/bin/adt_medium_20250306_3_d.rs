use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize,
        c: usize,
        mut b: [Chars; r],
    }

    fn bomb(i: usize, j: usize, e: u32, b: &mut Vec<Vec<char>>) {
        let r = b.len();
        let c = b[0].len();
        let e = e as usize;
        b[i][j] = '.';
        for di in 0..=e {
            for dj in 0..=(e-di) {
                if i + di < r {
                    let i2 = i + di;
                    if j + dj < c {
                        let j2 = j + dj;
                        if b[i2][j2] == '#' { b[i2][j2] = '.'; }
                    }
                    if j >= dj {
                        let j2 = j - dj;
                        if b[i2][j2] == '#' { b[i2][j2] = '.'; }
                    }
                }
                if i >= di {
                    let i2 = i - di;
                    if j + dj < c {
                        let j2 = j + dj;
                        if b[i2][j2] == '#' { b[i2][j2] = '.'; }
                    }
                    if j >= dj {
                        let j2 = j - dj;
                        if b[i2][j2] == '#' { b[i2][j2] = '.'; }
                    }
                }
            }
        }

    }
    for i in 0..r {
        for j in 0..c {
            if b[i][j] != '.' && b[i][j] != '#' {

                bomb(i, j, b[i][j].to_digit(10).unwrap(), &mut b);
            }
        }
    }
    for bi in b.iter() {
        println!("{}", bi.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}