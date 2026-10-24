use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            xy: [(isize, isize); n],
        }
        let mut l_cnt = 0;
        let mut r_cnt = 0;
        let mut l_xy: Vec<(isize, isize)> = Vec::new();
        let mut r_xy: Vec<(isize, isize)> = Vec::new();
        for &(x, y) in &xy {
            if x >= 0 {
                r_cnt += 1;
                r_xy.push((x, y));
            } else {
                l_cnt += 1;
                l_xy.push((x, y));
            }
        }
        let mut ans = l_cnt.max(r_cnt);
        r_xy.sort_by(|a, b| (a.1*b.0).cmp(&(b.1*a.0)));
        l_xy.sort_by(|a, b| (a.1*b.0).cmp(&(b.1*a.0)));
        let mut ri = 0;
        let mut li = 0;
        while ri < r_xy.len() || li < l_xy.len() {
            let (r_x, r_y) = if ri < r_xy.len() { r_xy[ri] } else { (0, 0) };
            let (l_x, l_y) = if li < l_xy.len() { l_xy[li] } else { (0 ,0) };
            // println!("ans: {}, r_cnt: {}, l_cnt: {}, r: ({}, {}), l: ({}, {})", ans, r_cnt, l_cnt, r_x, r_y, l_x, l_y);
            if li == l_xy.len() || r_y*l_x > l_y*r_x {  // rの方が近い
                r_cnt -= 1;
                l_cnt += 1;
                ri += 1;
            } else {
                r_cnt += 1;
                l_cnt -= 1;
                li += 1;
            }
            ans = ans.max(l_cnt).max(r_cnt);
        }
        println!("{}", ans);
    }
}