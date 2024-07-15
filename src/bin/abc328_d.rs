use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans: Vec<char> = Vec::new();
    for si in s.iter() {
        ans.push(*si);
        if ans.len() > 2 && ans[ans.len()-3] == 'A' && ans[ans.len()-2] == 'B' && ans[ans.len()-1] == 'C' {
            ans.pop();
            ans.pop();
            ans.pop();
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));

    /*
    let mut del: Vec<(usize, usize)> = Vec::new();  // 削除対象(インデックス, 長さ)
    let mut stack: Vec<(usize, usize)> = Vec::new();  // 削除候補(インデックス, 長さ)
    let mut i = 0;
    while i < s.len() && s.len() > 2 {
        if i < s.len()-2 && s[i] == 'A' && s[i+1] == 'B' && s[i+2] == 'C' {
            del.push((i, 3));
            i += 3;
            continue;
        }
        if s[i] == 'A' && i < s.len()-2 {
            if s[i+1] == 'B' {
                stack.push((i, 2));
                i += 2;
                continue;
            } else {
                stack.push((i, 1));
                i += 1;
                continue;
            }
        }
        if s[i] == 'B' {
            if stack.is_empty() {
                i += 1;
                continue;
            }
            let (j, c) = stack.pop().unwrap();
            if c == 1 && i < s.len()-1 && s[i+1] == 'C' {
                del.push((j, c));
                del.push((i, 2));
                i += 2;
                continue;
            }
        }
        if s[i] == 'C' {
            if stack.is_empty() {
                i += 1;
                continue;
            }
            let (j, c) = stack.pop().unwrap();
            if c == 2 {
                del.push((j, c));
                del.push((i, 1));
                i += 1;
                continue;
            }
        }
        stack = vec![];
        i += 1;
    }
    del.sort();
    println!("{:?}", del);
    let mut ans: Vec<char> = Vec::new();
    let mut j = 0;
    let (mut del_i, mut del_cnt) = (s.len(), 0);
    if j < del.len() {
        (del_i, del_cnt) = del[j];
        j += 1;
    }
    for i in 0..s.len() {
        if del_i + del_cnt <= i && j < del.len() {
            (del_i, del_cnt) = del[j];
            j += 1;
        }
        if del_i <= i && i < del_i + del_cnt {
            continue;
        } else {
            ans.push(s[i]);
        }
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    */
}