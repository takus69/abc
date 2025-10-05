use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            st: (f64, f64),
            gt: (f64, f64),
            sa: (f64, f64),
            ga: (f64, f64),
        }

        fn dist(t: (f64, f64), a: (f64, f64)) -> f64 {
            ((t.0-a.0)*(t.0-a.0) + (t.1-a.1)*(t.1-a.1)).sqrt()
        }
        fn goal(n: (f64, f64), n2: (f64, f64), g: (f64, f64)) -> bool {
            let nx = n.0.min(n2.0);
            let nx2 = n.0.max(n2.0);
            let ny = n.1.min(n2.1);
            let ny2 = n.1.max(n2.1);
            nx <= g.0 && g.0 <= nx2 && ny <= g.1 && g.1 <= ny2
        }
        fn r#move(s: (f64, f64), g: (f64, f64), d: (f64, f64), time: f64, max_time: f64) -> (f64, f64) {
            if time >= max_time {
                g
            } else {
                (s.0 + d.0*time, s.1 + d.1*time)
            }
        }

        let lt = dist(st, gt);
        let dt = ((gt.0-st.0)/lt, (gt.1-st.1)/lt);
        let la = dist(sa, ga);
        let da = ((ga.0-sa.0)/la, (ga.1-sa.1)/la);

        let mut l_time = 0.0;
        let mut r_time = lt.min(la);
        let nt = r#move(st, gt, dt, r_time, lt);
        let na = r#move(sa, ga, da, r_time, la);
        let mut l_dist = dist(st, sa);
        let mut r_dist = dist(nt, na);
        let mut ans = l_dist.min(r_dist);
        while l_time+0.000_000_000_1 < r_time {
            let m_time1 = (2.0*l_time+r_time) / 3.0;
            let m_time2 = (l_time+2.0*r_time) / 3.0;
            let nt1 = r#move(st, gt, dt, m_time1, lt);
            let na1 = r#move(sa, ga, da, m_time1, la);
            let nt2 = r#move(st, gt, dt, m_time2, lt);
            let na2 = r#move(sa, ga, da, m_time2, la);
            let m_dist1 = dist(nt1, na1);
            let m_dist2 = dist(nt2, na2);
            if m_dist1 > m_dist2 {
                l_dist = m_dist1;
                l_time = m_time1;
            } else {
                r_dist = m_dist2;
                r_time = m_time2;
            }
            ans = ans.min(r_dist).min(l_dist);
        }

        let mut l_time = lt.min(la);
        let mut r_time = 1000.0;
        let nt = r#move(st, gt, dt, l_time, lt);
        let na = r#move(sa, ga, da, l_time, la);
        let mut l_dist = dist(nt, na);
        let mut r_dist = dist(gt, ga);
        while l_time+0.000_000_000_1 < r_time {
            let m_time1 = (2.0*l_time+r_time) / 3.0;
            let m_time2 = (l_time+2.0*r_time) / 3.0;
            let nt1 = r#move(st, gt, dt, m_time1, lt);
            let na1 = r#move(sa, ga, da, m_time1, la);
            let nt2 = r#move(st, gt, dt, m_time2, lt);
            let na2 = r#move(sa, ga, da, m_time2, la);
            let m_dist1 = dist(nt1, na1);
            let m_dist2 = dist(nt2, na2);
            if m_dist1 > m_dist2 {
                l_dist = m_dist1;
                l_time = m_time1;
            } else {
                r_dist = m_dist2;
                r_time = m_time2;
            }
            ans = ans.min(r_dist).min(l_dist);
        }
        println!("{}", ans);

    }
}