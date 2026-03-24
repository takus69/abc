use proconio::input;
use ac_library::FenwickTree;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        q: usize,
    }
    // 同じaiの数を数える
    let mut cnt: Vec<usize> = vec![0; m+1];
    for &ai in &a {
        cnt[ai] += 1;
    }
    // 1～mに対して、出現回数を記録
    let mut cnt_list: Vec<(usize, usize)> = Vec::new();
    for i in 1..=m {
        cnt_list.push((cnt[i], i));
    }
    // 出現回数が少ない数から追加
    cnt_list.sort();
    // 出現回数が同じ追加を次に多い出現回数まで追加するのを1回として、k回目の追加で何個になるか
    let mut k: Vec<usize> = Vec::new();
    let mut c: usize = 0;
    let mut prev = usize::MAX;
    for &(v, _) in &cnt_list {
        if prev != v {
            k.push(c);
            c = 0;
            prev = v;
        }
        c += 1;
    }
    // k回目の追加での累積和
    let mut sum = n;
    let mut cumsum: Vec<usize> = vec![0];
    let mut prev = 0;
    for (k, &(v, _)) in cnt_list.iter().enumerate() {
        sum += (v-prev)*k;
        cumsum.push(sum);
        prev = v;
    }
    
    let mut key: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..q {
        input! {
            x: usize,
        }
        // xのクエリが、k回目の追加で、k回目の追加のv番目の数
        let k = cumsum.partition_point(|&y| y < x)-1;
        let mut v = x-cumsum[k];
        if k > 0 {
            v = (v-1)%k+1;
        }
        // println!("x: {}, k: {}, v: {}, i: {}", x, k, v, i);
        key.push((k, v, i));
    }
    key.sort();
    // println!("cumsum: {:?}", cumsum);
    // println!("key: {:?}", key);
    // println!("cnt_list: {:?}", cnt_list);

    let mut ans: Vec<(usize, usize)> = Vec::new();
    let mut fw = FenwickTree::new(m+1, 0);
    let mut idx = 0;
    for &(k, v, i) in &key {
        if k == 0 {
            ans.push((i, a[v-1]));
            continue;
        }
        while idx < cnt_list.len() && k > idx {
            fw.add(cnt_list[idx].1, 1);
            // println!("add: {}, sum: {}", cnt_list[idx].1, fw.sum(0..=4));
            idx += 1;
        }
        let mut ok = 0;
        let mut ng = m+1;
        while ok+1 < ng {
            let mid = (ok+ng)/2;
            if fw.sum(0..=mid) < v {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        // println!("k: {}, v: {}, idx: {}, ok: {}", k, v, idx, ok);
        ans.push((i, ok+1));
    }
    ans.sort();
    for (_, v) in ans {
        println!("{}", v);
    }
}