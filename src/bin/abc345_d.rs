use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n],
    }

    // 再帰的にabを追加していく
    fn add_ab(mut sets: Vec<Vec<bool>>, target: &Vec<(usize, usize)>, perm: &Vec<usize>, t: usize) -> bool {
        if perm.len() == t { return true; }
        let mut ret = true;
        let h = sets.len();
        let w = sets[0].len();
        // println!("target: {:?}, perm: {:?}, t: {}", target, perm, t);
        // 右上から置ける場所を探す
        let (mut start_i, mut start_j) = (99, 99);
        for i in 0..h {
            for j in 0..w {
                if !sets[i][j] {
                    start_i = i;
                    start_j = j;
                    break;
                }
            }
            if start_i < 99 { break; }
        }
        // perm.pop()のabを置く(縦横の2パターン)
        let (a, b) = target[perm[t]];

        let mut flg = true;
        let mut sets2 = sets.clone();
        // println!("start_i: {}, start_j: {}, h: {}, w: {}", start_i, start_j, h, w);
        for i in 0..a {
            for j in 0..b {
                if start_i + i < h && start_j + j < w && !sets2[start_i+i][start_j+j] {
                    sets2[start_i+i][start_j+j] = true;
                } else {
                    // println!("i: {}, j: {}, break", i, j);
                    flg = false;
                    break;
                }
            }
            if !flg { break; }
        }
        if flg {
            flg = add_ab(sets2, &target, &perm, t+1);
            if flg {
                println!("Yes");
                std::process::exit(0);
            }
        }
        ret = flg;

        let mut flg = true;
        let mut sets2 = sets.clone();
        for i in 0..b {
            for j in 0..a {
                if start_i + i < h && start_j + j < w && !sets2[start_i+i][start_j+j] {
                    sets2[start_i+i][start_j+j] = true;
                } else {
                    flg = false;
                    break;
                }
            }
            if !flg { break; }
        }
        if flg {
            flg = add_ab(sets2, &target, &perm, t+1);
            if flg {
                println!("Yes");
                std::process::exit(0);
            }
        }
        ret |= flg;

        ret
    }

    // どれを使うか: 2^n(最大2^7=128)
    for i in 1..(2_usize.pow(n as u32)) {
        let mut sum_ab = 0;
        let mut target: Vec<(usize, usize)> = Vec::new();
        for j in 0..n {
            if i >> j & 1 == 1 {
                sum_ab += ab[j].0 * ab[j].1;
                target.push(ab[j]);
            }
        }
        // abの面積の合計 == h*w
        if sum_ab != h*w { continue; }
        // どう配置するか: 7!*2^7=6*10^5
        // println!("target: {:?}", target);
        for perm in (0..target.len()).permutations(target.len()) {
            // println!("perm: {:?}", perm);
            let sets = vec![vec![false; w]; h];
            add_ab(sets.clone(), &target, &perm, 0);
        }
    }
    println!("No");
}