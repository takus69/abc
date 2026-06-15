use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            a: [usize; 3],
        }
        /*
        // 愚直
        let mut vec: Vec<usize> = vec![0; a[0]];
        vec.extend(vec![1; a[1]]);
        vec.extend(vec![2; a[2]]);
        let mut ans = 0;
        let n = vec.len();
        for perm in (0..n).permutations(n) {
            let mut tmp_ans = 0;
            // println!("perm: {:?}", perm);
            for (i, &i1) in perm.iter().enumerate() {
                let i2 = perm[(i+1)%n];
                let i3 = perm[(n+i-1)%n];
                if (vec[i1]+1)%3 == vec[i2] && (vec[i1]+1)%3 == vec[i3] {
                    // println!("i: {}, i2: {}, i3: {}, vec[i]: {}, vec[i2]: {}, vec[i3]: {}", i, i2, i3, vec[i], vec[i2], vec[i3]);
                    tmp_ans += 1;
                }
            }
            if tmp_ans == 3 {
                println!("perm: {:?}", perm);
                // break;
            }
            ans = ans.max(tmp_ans);
        }
        println!("ans: {}", ans);
        */
        // 1周する場合は事前計算
        let mut ans = if (a[0]==0&&a[1]==a[2]) || (a[1]==0&&a[2]==a[0]) || (a[2]==0&&a[0]==a[1]) { *a.iter().max().unwrap() } else { 0 };
        for perm in (0..3).permutations(3) {
            let mut tmp = a.clone();
            if tmp[0] > 0 && tmp[1] > 0 && tmp[2] > 0 {
                tmp.sort();
                let (x, y, z) = (tmp[0]-1, tmp[1]-1, tmp[2]-1);
                let tmp_ans = if x+y < z { x+y } else { (x+y+z)/2 };
                ans = ans.max(tmp_ans);
            }

            // 3つ勝つ
            let mut tmp = a.clone();
            let mut tmp_ans = 0;
            for &i in &perm {
                let w = i;
                let l = (i+1)%3;
                if tmp[w]==0 || tmp[l]==0 { continue; }
                let c = tmp[w].min(tmp[l]-1);
                tmp[w] -= c;
                tmp[l] -= c+1;
                tmp_ans += c;
                // println!("tmp: {:?}, tmp_ans: {}", tmp, tmp_ans);
            }
            // println!("tmp_ans: {}, perm: {:?}", tmp_ans, perm);
            ans = ans.max(tmp_ans);
        }
        println!("{}", ans);
    }
}