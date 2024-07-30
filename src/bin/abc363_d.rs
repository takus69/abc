use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut cnt = 1;
    let mut last_cnt = 1;
    let mut i = 0;
    while cnt < n {
        i += 1;
        let tmp = 9 * 10_usize.pow(i/2+i%2-1);
        cnt += tmp;
        last_cnt = tmp;
    }
    if n == 1 {
        println!("0");
        std::process::exit(0);
    }

    let mut ans: Vec<usize> = Vec::new();
    let digit = i/2+i%2;
    // n桁目を決定
    let diff = 10_usize.pow(digit-1);
    let mut del_cnt = 0;
    for k in (1..=9).rev() {
        if diff <= cnt - n {
            del_cnt += diff;
            cnt -= diff;
        } else {
            ans.push(k);
            break;
        }
    }

    // n-1桁目以降を決定
    for j in 1..digit {
        let diff = 10_usize.pow(digit-j-1);
        let mut del_cnt = 0;
        for k in (0..=9).rev() {
            if diff <= cnt - n {
                del_cnt += diff;
                cnt -= diff;
            } else {
                ans.push(k);
                break;
            }
        }
    }
    let mut ans2 = Vec::new();
    for a in ans.iter() {
        ans2.push(a.to_string());
    }
    for (j, a) in ans.iter().rev().enumerate() {
        if i%2 == 1 && j == 0 {
            continue;
        }
        ans2.push(a.to_string());
    }
    println!("{}", ans2.join(""));
}