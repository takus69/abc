use proconio::input;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        k: usize,
        abc: [(usize, usize, usize); n],
    }
    let mut ans: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut t = 0;
    let mut cnt = 0;
    let mut eating: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut waiting: VecDeque<(usize, usize, usize)> = VecDeque::new();
    // for &(ai, bi, ci) in abc.iter() {
    while !waiting.is_empty() || i < n {
        let ai = if i < n {
            let (ai, _, _) = abc[i];
            ai
        } else { usize::MAX };
        let ti = if !eating.is_empty() {
            let &(Reverse(ti), _) = eating.peek().unwrap();
            ti
        } else {
            usize::MAX
        };
        if ai <= ti {  // 待ち行列追加
            let (ai, bi, ci) = abc[i];
            waiting.push_front((ai, bi, ci));
            i += 1;
            t = ai;
        } else {  // 退店
            let &(Reverse(ti), ci) = eating.peek().unwrap();
            eating.pop();
            cnt -= ci;
            t = ti;
        }

        // 入店
        while cnt < k && !waiting.is_empty() {
            let &(_, bi, ci) = waiting.iter().last().unwrap();
            if cnt + ci <= k {
                eating.push((Reverse(t+bi), ci));
                waiting.pop_back();
                cnt += ci;
                ans.push(t);
            } else {
                break;
            }
        }

    }
    for ai in ans.iter() {
        println!("{}", ai);
    }
}