use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    }
    let mut prefix: Vec<isize> = vec![0; n+1];
    let mut pos: Vec<isize> = vec![0; n+1];
    for i in 0..n {
        prefix[i+1] += prefix[i] + a[i];
        pos[i+1] += pos[i] + (i+1) as isize;
    }
    let mut prefix2: Vec<isize> = vec![0; n+1];
    for i in 0..n {
        prefix2[i+1] += prefix2[i] + prefix[i+1];
    }
    // println!("prefix: {:?}", prefix);
    // println!("prefix2: {:?}", prefix2);
    // println!("pos: {:?}", pos);
    for _ in 0..q {
        input! {
            l: isize,
            r: isize,
        }
        let li = pos.partition_point(|&x| x < l);
        let ri = pos.partition_point(|&x| x < r);
        // println!("ri: {}, li: {}", ri, li);
        let mut ans = prefix2[ri] - prefix2[li];
        // println!("{} - {}, {}, {}", prefix[li], prefix[l-li*(li-1)/2-1], li, l-li*(li-1)/2-1);
        // println!("{} - {}, {}, {}", prefix[ri], prefix[r-ri*(ri-1)/2-1], ri, r-ri*(ri-1)/2-1);
        ans += prefix[li] - prefix[l as usize -li*(li-1)/2-1];
        ans -= prefix[ri] - prefix[r as usize -ri*(ri-1)/2];
        println!("{}", ans);
    }
}