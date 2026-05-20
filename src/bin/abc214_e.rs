use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut lr: [(usize, usize); n],
        }
        lr.sort();
        let mut r_list: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
        let mut pre_l = 0;
        let mut ok = true;
        for &(l, r) in &lr {
            // println!("l: {}, r: {}, r_list: {}", l, r, r_list.len());
            if pre_l != l {
                for c in 0..(l-pre_l) {
                    if let Some(Reverse(ri)) = r_list.pop() {
                        // println!("ri: {}, c: {}, l: {}", ri, c, l);
                        if ri < c+pre_l {
                            ok = false;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            r_list.push(Reverse(r));
            pre_l = l;
        }
        // 最後のチェック
        // println!("r_list: {}", r_list.len());
        for c in 0..r_list.len() {
            if let Some(Reverse(ri)) = r_list.pop() {
                // println!("ri: {}, c: {}, l: {}", ri, c, pre_l);
                if ri < c+pre_l {
                    ok = false;
                    break;
                }
            } else {
                ok = false;
                break;
            }
        }
        if ok { println!("Yes"); } else { println!("No"); }
    }
}