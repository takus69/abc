use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut vac: [(usize, isize, usize); n],
    }

    // ビタミン1
    let mut vac2 = vec![];
    for &(v, a, c) in vac.iter() {
        if v == 1 {
            vac2.push((v, a, c));
        }
    }
    let mut dp1 = vec![vec![-1; x+1]; vac2.len()+1];
    dp1[0][0] = 0;
    for (i, &(_, a, c)) in vac2.iter().enumerate() {
        for j in 0..(x+1) {
            dp1[i+1][j] = dp1[i+1][j].max(dp1[i][j]);
            if j+c > x { continue; }
            if dp1[i][j] < 0 { continue; }
            dp1[i+1][j+c] = dp1[i+1][j+c].max(dp1[i][j] + a);
        } 
    }

    // ビタミン2
    let mut vac2 = vec![];
    for &(v, a, c) in vac.iter() {
        if v == 2 {
            vac2.push((v, a, c));
        }
    }
    let mut dp2 = vec![vec![-1; x+1]; vac2.len()+1];
    dp2[0][0] = 0;
    for (i, &(_, a, c)) in vac2.iter().enumerate() {
        for j in 0..(x+1) {
            dp2[i+1][j] = dp2[i+1][j].max(dp2[i][j]);
            if j+c > x { continue; }
            if dp2[i][j] < 0 { continue; }
            dp2[i+1][j+c] = dp2[i+1][j+c].max(dp2[i][j] + a);
        } 
    }

    // ビタミン3
    let mut vac2 = vec![];
    for &(v, a, c) in vac.iter() {
        if v == 3 {
            vac2.push((v, a, c));
        }
    }
    let mut dp3 = vec![vec![-1; x+1]; vac2.len()+1];
    dp3[0][0] = 0;
    for (i, &(_, a, c)) in vac2.iter().enumerate() {
        for j in 0..(x+1) {
            dp3[i+1][j] = dp3[i+1][j].max(dp3[i][j]);
            if j+c > x { continue; }
            if dp3[i][j] < 0 { continue; }
            dp3[i+1][j+c] = dp3[i+1][j+c].max(dp3[i][j] + a);
        } 
    }

    // println!("dp1: {:?}", dp1.last().unwrap());
    // println!("dp2: {:?}", dp2.last().unwrap());
    // println!("dp3: {:?}", dp3.last().unwrap());

    let mut tmp_max = 0;
    let mut max_dp1: Vec<isize> = Vec::new();
    for &vi in dp1[dp1.len()-1].iter() {
        tmp_max = tmp_max.max(vi);
        max_dp1.push(tmp_max);
    }

    let mut tmp_max = 0;
    let mut max_dp2: Vec<isize> = Vec::new();
    for &vi in dp2[dp2.len()-1].iter() {
        tmp_max = tmp_max.max(vi);
        max_dp2.push(tmp_max);
    }

    let mut tmp_max = 0;
    let mut max_dp3: Vec<isize> = Vec::new();
    for &vi in dp3[dp3.len()-1].iter() {
        tmp_max = tmp_max.max(vi);
        max_dp3.push(tmp_max);
    }

    // println!("max_dp1: {:?}", max_dp1);
    // println!("max_dp2: {:?}", max_dp2);
    // println!("max_dp3: {:?}", max_dp3);

    let (mut m1, mut m2, mut m3) = (0, 0, 0);
    let (mut x1, mut x2, mut x3) = (0, 0, 0);
    let mut ans: isize = 0;
    while x1 + x2 + x3 < x {
        // println!("{} {} {} {}", m1, m2, m3, ans);
        if m1 <= m2 && m1 <= m3 {
            x1 += 1;
            m1 = max_dp1[x1];
        } else if m2 <= m1 && m2 <= m3 {
            x2 += 1;
            m2 = max_dp2[x2];
        } else {
            x3 += 1;
            m3 = max_dp3[x3];
        }
        ans = ans.max(m1.min(m2).min(m3));

    }

    println!("{}", ans);

}