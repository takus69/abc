use proconio::input;
use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }
    let mut link: HashMap<String, Vec<String>> = HashMap::new();
    for (s, t) in st.iter() {
        let e = link.entry(s.clone()).or_insert(vec![]);
        e.push(t.clone());
    }
    let mut q: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut ans = "Yes";
    for (s, t) in st.iter() {
        if visited.contains(s) { continue; }
        visited.insert(s.clone());
        let mut same: HashSet<String> = HashSet::new();
        same.insert(s.clone());
        q.push_front(s.clone());
        while !q.is_empty() {
            let s = q.pop_back().unwrap();
            if !link.contains_key(&s) { continue; }
            let e = link.get(&s).unwrap();
            if e.len() > 1 { println!("No"); return; }
            for t in e.iter() {
                if same.contains(t) { println!("No"); return; }
                q.push_front(t.clone());
                visited.insert(t.clone());
                same.insert(t.clone());
            }
        }
    }
    println!("Yes");
}