use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for (i, ai) in a.iter().enumerate() {
        map.insert(i, ai-1);
    }
    let mut ans = 0;
    let mut visited = vec![false; n];
    let mut i = x-1;
    while !visited[i] {
        ans += 1;
        visited[i] = true;
        i = *map.get(&i).unwrap();
    }

    println!("{}", ans);
}