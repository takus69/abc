use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        mut td: [(usize, usize); n],
    }
    td.sort();
    let mut ans: usize = 0;
    let mut now: usize = 1;
    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut i = 0;
    while i < n || !heap.is_empty() {
        let (t, d) = if i == n { (0, 0) } else { td[i] };
        // println!("now1: {}, t: {}, d: {}, ans: {}, heap: {:?}", now, t, d, ans, heap);
        // 現時点までに入る商品は全て入れる
        if t == now {
            heap.push(Reverse(t+d));
            i += 1;
            continue;
        } else if now < t && heap.is_empty() {
            now = t;
        }
        // 印字できる商品を印字する
        while let Some(Reverse(min_d)) = heap.peek() {
            if now <= *min_d {
                heap.pop();
                ans += 1;
                now += 1;
                break;
            } else {
                heap.pop();
            }
        }
        // println!("now2: {}, t: {}, d: {}, ans: {}, heap: {:?}", now, t, d, ans, heap);
    }

    println!("{}", ans);
}