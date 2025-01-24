use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        mut s: Chars,
    }
    let mut map: HashMap<char, usize> = HashMap::new();
    map.insert('A', 0);
    map.insert('B', 0);
    map.insert('C', 0);
    let mut ans = "Yes";
    for &si in s.iter() {
        let e = map.get_mut(&si).unwrap();
        *e += 1;
        if map[&'B'] > 0 && map[&'C'] == 0 && si != 'B' { ans = "No"; break; }
        if map[&'C'] > 0 && si != 'C' { ans = "No"; break; }
    }
    s.sort();
    if s[0] == s[s.len()-1] { ans = "Yes"; }
    println!("{}", ans);
}