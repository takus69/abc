use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
    }
    let mut tmp: HashMap<char, usize> = HashMap::new();
    for &ci in x.iter().chain(y.iter()) {
        tmp.insert(ci, 0);
    }
    let mut cnt: Vec<HashMap<char, usize>> = vec![tmp.clone(); 2];
    let mut len : Vec<usize> = vec![x.len(), y.len()];
    for &xi in &x {
        let e = cnt[0].entry(xi).or_insert(0);
        *e += 1;
    }
    for &yi in &y {
        let e = cnt[1].entry(yi).or_insert(0);
        *e += 1;
    }
    let mut sum_x: HashMap<char, Vec<usize>> = HashMap::new();
    for &ci in x.iter().chain(y.iter()) {
        sum_x.insert(ci, vec![0]);
    }
    let mut sum_y = sum_x.clone();
    let keys = sum_x.keys().cloned().collect::<Vec<char>>();
    for &xi in &x {
        for &k in &keys {
            let e = sum_x.get_mut(&k).unwrap();
            let l = *(e.last().unwrap());
            if k == xi {
                e.push(l+1);
            } else {
                e.push(l);
            }
        }
    }
    for &yi in &y {
        for &k in &keys {
            let e = sum_y.get_mut(&k).unwrap();
            let l = *(e.last().unwrap());
            if k == yi {
                e.push(l+1);
            } else {
                e.push(l);
            }
        }
    }
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            c: char,
        }
        while len.last().unwrap() < &r {
            let i = len.len();
            let mut tmp = cnt[i-1].clone();
            for (k, v) in cnt[i-2].iter() {
                let e = tmp.get_mut(k).unwrap();
                *e += v;
            }
            cnt.push(tmp);
            len.push(len[i-1]+len[i-2]);
        }

        // println!("len: {:?}", len);
        
        if !sum_x.contains_key(&c) {
            println!("0");
        } else {
            println!("{}", ans(len.len()-1, l, r, c, &cnt, &len, &sum_x, &sum_y));
        }
    }

    fn ans(i: usize, l: usize, r: usize, c: char, cnt: &Vec<HashMap<char, usize>>, len: &Vec<usize>, sum_x: &HashMap<char, Vec<usize>>, sum_y: &HashMap<char, Vec<usize>>) -> usize {
        // println!("i: {}, l: {}, r: {}", i, l, r);
        if l==1 && r==len[i] { return *(cnt[i].get(&c).unwrap()); }
        if i == 0 {
            let sum = sum_x.get(&c).unwrap();
            return sum[r]-sum[l-1];
        }
        if i == 1 {
            let sum = sum_y.get(&c).unwrap();
            return sum[r]-sum[l-1];
        }
        let len1 = len[i-1];
        let len2 = len[i-2];
        if len1 < l {
            return ans(i-2, l-len1, r-len1, c, cnt, len, sum_x, sum_y);
        } else if r > len1 {
            return ans(i-1, l, len1, c, cnt, len, sum_x, sum_y) + ans(i-2, 1, r-len1, c, cnt, len, sum_x, sum_y);
        } else {
            return ans(i-1, l, r, c, cnt, len, sum_x, sum_y);
        }
    }
}