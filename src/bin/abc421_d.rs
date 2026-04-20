use proconio::input;

fn main() {
    input! {
        mut t: (isize, isize),
        mut a: (isize, isize),
        n: usize,
        m: usize,
        l: usize,
        sa: [(char, isize); m],
        tb: [(char, isize); l],
    }

    // 現在のターン
    let mut tt = 0;
    let mut ta = 0;
    // 現在ターンの残り手数
    let mut lt = sa[tt].1;
    let mut la = tb[ta].1;
    // 初期位置の処理
    let mut ans = 0;
    while tt < m || ta < l {
        ans += apply(&mut t, &mut a, &mut lt, &mut la, sa[tt].0, tb[ta].0);
        // println!("ans: {}", ans);
        if lt == 0 {
            tt += 1;
            if tt < m {
                lt = sa[tt].1;
            }
        }
        if la == 0 {
            ta += 1;
            if ta < l {
                la = tb[ta].1;
            }
        }
    }
    println!("{}", ans);

    fn apply(t: &mut (isize, isize), a: &mut (isize, isize), lt: &mut isize, la: &mut isize, tdir: char, adir: char) -> isize {
        let pre_t = *t;
        let pre_a = *a;
        let len = (*lt).min(*la);
        *t = r#move(*t, tdir, len);
        *a = r#move(*a, adir, len);
        if *lt <= *la {
            *la -= *lt;
            *lt = 0;
        } else {
            *lt -= *la;
            *la = 0;
        }

        // println!("t: {:?}=>{:?}, a: {:?}=>{:?}, lt: {}, la: {}", pre_t, *t, pre_a, *a, *lt, *la);
        if pre_t == pre_a && *t == *a {
            len
        } else if same(pre_t, pre_a, *t, *a, tdir, adir, len) {
            1
        } else {
            0
        }
    }

    fn r#move(p: (isize, isize), dir: char, cnt: isize) -> (isize, isize) {
        let mut c = p.1;
        let mut r = p.0;
        match dir {
            'L' => { c -= cnt },
            'R' => { c += cnt },
            'U' => { r -= cnt },
            'D' => { r += cnt },
            _ => {},
        }
        (r, c)
    }

    fn reachable(t: (isize, isize), a: (isize, isize), tdir: char, adir: char) -> isize {
        let tlen = match tdir {
            'L' => t.1 - a.1,
            'R' => a.1 - t.1,
            'U' => t.0 - a.0,
            'D' => a.0 - t.0,
            _ => 0,
        };
        let alen = match adir {
            'L' => a.1 - t.1,
            'R' => t.1 - a.1,
            'U' => a.0 - t.0,
            'D' => t.0 - a.0,
            _ => 0,
        };
        // println!("same: t: {:?}, a: {:?}, tdir: {}, adir: {}, tlen: {}, alen: {}", t, a, tdir, adir, tlen, alen);

        if alen > 0 && tlen > 0 {
            let len = alen.max(tlen);
            if opp(tdir, adir) {
                let same_row_col = match tdir {
                    'L' | 'R' => a.0 == t.0,
                    'U' | 'D' => a.1 == t.1,
                    _ => false,
                };
                if same_row_col && len % 2 == 0 {
                    tlen/2
                } else {
                    isize::MAX
                }
            } else if tdir == adir {
                isize::MAX
            } else if alen == tlen {
                len
            } else {
                isize::MAX
            }
        } else {
            isize::MAX
        }
    }
    
    fn opp(d1: char, d2: char) -> bool {
        (d1 == 'L' && d2 == 'R') || (d1 == 'R' && d2 == 'L') || (d1 == 'U' && d2 == 'D') || (d1 == 'D' && d2 == 'U')
    }

    fn same(pre_t: (isize, isize), pre_a: (isize, isize), t: (isize, isize), a: (isize, isize), tdir: char, adir: char, len: isize) -> bool {
        let len2 = reachable(pre_t, pre_a, tdir, adir);
        len2 != isize::MAX && len2 <= len
    }
}
