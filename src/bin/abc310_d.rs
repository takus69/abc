use proconio::input;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut link: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 1..=n {
        link.insert(i, vec![]);
    }
    for (a, b) in ab.iter() {
        let e = link.entry(*a).or_insert(vec![]);
        e.push(*b);
        let e = link.entry(*b).or_insert(vec![]);
        e.push(*a);
    }

    fn add_team(teams: Vec<Vec<usize>>, c: usize, t: usize, link: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = Vec::new();
        let l = link.get(&(c)).unwrap();
        for team in teams.iter() {
            let mut max_t = *(team.iter().max().unwrap());
            if max_t < t { max_t += 1; }
            for i in 1..=max_t {  // cが追加されるteamの候補
                let mut flg = false;
                for j in l.iter() {
                    if team[j-1] == i {  // 相性の悪い人がいたら除外
                        flg = true;
                        break;
                    }  
                }
                if flg { continue; }
                let mut tmp = team.clone();
                tmp[c-1] = i;
                ret.push(tmp);
            }
        }

        ret
    }

    // 1～i-1でのチームがある
    // 上記にiを追加するor tチーム未満なら新しいチームを作る
    let mut teams: Vec<Vec<usize>> = vec![vec![0; n]];
    for i in 1..=n {
        teams = add_team(teams, i, t, &link);
    }
    let mut ans = 0;
    for team in teams.iter() {
        if team.iter().max().unwrap() == &t {
            ans += 1;
        }
    }

    println!("{}", ans);
}