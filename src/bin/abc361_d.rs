use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        mut n: usize,
        mut s: String,
        mut t: String,
    }

    let mut ans = -1;
    n += 2;
    s = format!("{}..", s);
    t = format!("{}..", t);

    // BFSですべてのあり得る状態を列挙する
    let mut map: HashMap<String, i64> = HashMap::new();
    map.insert(s.clone(), 0);
    let mut que: VecDeque<String> = VecDeque::new();
    que.push_front(s.clone());
    
    while !que.is_empty() {
        let c = que.pop_back().unwrap();
        let d = *map.get(&c).unwrap();
        if c == t {
            ans = d;
            break;
        }
        let mut empty: usize = 0;
        for i in 0..(n-1) {
            let c2: Vec<char> = c.chars().collect();
            if c2[i] == '.' {
                empty = i;
                break;
            }
        }
        for i in 0..(n-1) {
            let mut c2: Vec<char> = c.chars().collect();
            if c2[i] == '.' || c2[i+1] == '.' {
                continue;
            }
            // swap
            c2[empty] = c2[i];
            c2[empty+1] = c2[i+1];
            c2[i] = '.';
            c2[i+1] = '.';
            let s = c2.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
            if !map.contains_key(&s) {
                map.insert(s.clone(), d+1);
                que.push_front(s.clone());
            }
        }
    }

    println!("{}", ans);
}