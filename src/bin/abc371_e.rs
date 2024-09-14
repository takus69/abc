use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: Vec<Vec<usize>> = vec![vec![]; n+1];
    for (i, ai) in a.iter().enumerate() {
        map[*ai].push(i);
    }
    // println!("map: {:?}", map);
    let mut ans = 0;
    for ai in 1..=n {
        let mut tmp = n*(n+1)/2;
        let mut pre = 0;
        for i in map[ai].iter() {
            if pre == *i { pre += 1; continue; }
            let cnt = i - pre;
            tmp -= cnt*(cnt+1)/2;
            pre = i+1;
        }
        if n > pre {
            let cnt = n - pre;
            tmp -= cnt*(cnt+1)/2;
        }
        ans += tmp;
        // println!("ai: {}, ans: {}, tmp: {}", ai, ans, tmp);
    }
    println!("{}", ans);
}