use proconio::input;
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        sx: isize,
        sy: isize,
        xy: [(isize, isize); n],
        dc: [(char, isize); m],
    }
    let mut x_map: HashMap<isize, Vec<isize>> = HashMap::new();
    let mut y_map: HashMap<isize, Vec<isize>> = HashMap::new();
    for &(x, y) in xy.iter() {
        let e = x_map.entry(x).or_insert(vec![]);
        e.push(y);
        let e = y_map.entry(y).or_insert(vec![]);
        e.push(x);
    }
    for v in x_map.values_mut() {
        v.sort();
    }
    for v in y_map.values_mut() {
        v.sort();
    }

    let (mut x, mut y) = (sx, sy);
    let mut ans = 0;
    for &(d, c) in dc.iter() {
        match d {
            'U' => {
                if let Some(v) = x_map.get_mut(&x) {
                    let i = match v.binary_search(&(y)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    let i2 = match v.binary_search(&(y+c+1)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    ans += i2 - i;
                    for ii in i..i2 {
                        let vv = y_map.get_mut(&v[ii]).unwrap();
                        if vv.len() == 0 { continue; }
                        let j = vv.binary_search(&x).unwrap();
                        vv.drain(j..(j+1));
                    }
                    v.drain(i..i2);
                }
                y += c;
            },
            'D' => {
                if let Some(v) = x_map.get_mut(&x) {
                    let i = match v.binary_search(&(y-c)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    let i2 = match v.binary_search(&(y+1)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    ans += i2 - i;
                    for ii in i..i2 {
                        let vv = y_map.get_mut(&v[ii]).unwrap();
                        if vv.len() == 0 { continue; }
                        let j = vv.binary_search(&x).unwrap();
                        vv.drain(j..(j+1));
                    }
                    v.drain(i..i2);
                }
                y -= c;
            },
            'L' => {
                if let Some(v) = y_map.get_mut(&y) {
                    let i = match v.binary_search(&(x-c)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    let i2 = match v.binary_search(&(x+1)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    ans += i2 - i;
                    for ii in i..i2 {
                        let vv = x_map.get_mut(&v[ii]).unwrap();
                        if vv.len() == 0 { continue; }
                        let j = vv.binary_search(&y).unwrap();
                        vv.drain(j..(j+1));
                    }
                    v.drain(i..i2);
                }
                x -= c;
            },
            'R' => {
                if let Some(v) = y_map.get_mut(&y) {
                    let i = match v.binary_search(&(x)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    let i2 = match v.binary_search(&(x+c+1)) { Ok(xx) => { xx }, Err(xx) => { xx }};
                    ans += i2 - i;
                    for ii in i..i2 {
                        let vv = x_map.get_mut(&v[ii]).unwrap();
                        if vv.len() == 0 { continue; }
                        let j = vv.binary_search(&y).unwrap();
                        vv.drain(j..(j+1));
                    }
                    v.drain(i..i2);
                }
                x += c;
            },
            _ => {},
        }
    }
    println!("{} {} {}", x, y, ans);

}