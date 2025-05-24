use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut up = false;
    let mut down = false;
    let mut pre = p[0];
    let mut ups: Vec<usize> = Vec::new();
    let mut downs: Vec<usize> = Vec::new();
    for (i, &pi) in p.iter().enumerate().skip(1) {
        if pre < pi {
            if down {
                downs.push(i);
            }
            up = true;
            down = false;
            } else {
            if up {
                ups.push(i);
            }
            down = true;
            up = false;
            }
        pre = pi;
    }
    // println!("ups: {:?}", ups);
    // println!("downs: {:?}", downs);

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    let mut pre_i = 1;
    while i < ups.len() && j < downs.len() {
        let i2 = ups[i];
        let j2 = downs[j];
        let next_j = if i+1 < ups.len() { ups[i+1] } else { n };
        if i2 < j2 {
        ans += (i2 - pre_i) * (next_j - j2);
        i += 1;
        j += 1;
    } else {
    j += 1;
    }
    pre_i = j2;
    }
    println!("{}", ans);
}