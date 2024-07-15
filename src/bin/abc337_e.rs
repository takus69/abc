use proconio::{input_interactive, marker::Chars};

fn main() {
    input_interactive! {
        n: usize,
    }

    // 必要人数算出
    let mut m = 0;
    let mut cnt: Vec<usize> = vec![n];
    let mut start = 0;  // 前の階層のスタート位置
    let mut i = 0;  // 階層
    while true {
        let mut end_flg = true;
        for j in start..(start+(1<<i)) {  // 前の階層の数を分割していく
            let c1 = cnt[j] / 2;
            let c2 = cnt[j] - c1;
            if c1 > 1 || c2 > 1 { end_flg = false; }
            m += 1;
            cnt.push(c1);
            cnt.push(c2);
        }
        if end_flg { break; }
        start += 1<<i;
        i += 1;
    }
    println!("{:?}", cnt);
    println!("{}", m);
    
    // 飲ませるジュースを決定
    let mut start = 0;  // 前の階層のスタート位置
    let mut i = 0;  // 階層
    while true {
        let mut end_flg = true;
        let mut k = 1;
        for j in start..(start+(1<<i)) {  // 前の階層の数を分割していく
            //let c1 = cnt[j] / 2;
            //let c2 = cnt[j] - c1;
            let k2 = start+(1<<i) + j*2;
            println!("{}", k2);
            let c1 = cnt[k2];
            let c2 = cnt[k2+1];
            if c1 > 1 || c2 > 1 { end_flg = false; }
            println!("{} {}", c1, (k..(c1+k)).map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
            k += c1 + c2;
        }
        if end_flg { break; }
        start += 1<<i;
        i += 1;
    }
    
    // 翌日の結果から腐ったジュースを判定
    input_interactive! {
        s: Chars,
    }

    let mut start = 0;  // 前の階層のスタート位置
    let mut i = 0;  // 階層
    let mut l: usize = 0;
    let mut r: usize = n;
    let mut k = 0;
    while true {
        let mut end_flg = true;
        for j in start..(start+(1<<i)) {  // 前の階層の数を分割していく
            let k2 = start+(1<<i) + j*2;
            let c1 = cnt[k2];
            let c2 = cnt[k2+1];
            if c1 > 1 || c2 > 1 { end_flg = false; }
            if c1 > 1 {
                if s[k] == '1' {
                    l = j*2;
                    r = j*2+c1;
                } else {
                    l = j*2+c1;
                    r = j*2+c1+c2;
                }
                k += 2;
                end_flg = false;
            } else if c1 == 1 {
                if s[k] == '0' {
                    l = j*2;
                    r = j*2+c1;
                } else {
                    l = j*2+c1;
                    r = j*2+c1+c2;
                }
                k += 1;
            }
        }
        if end_flg { break; }
        start += 1<<i;
        i += 1;
    }
    println!("{}", l);
}