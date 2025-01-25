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
            if j+c > x { break; }
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
            if j+c > x { break; }
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
            if j+c > x { break; }
            if dp3[i][j] < 0 { continue; }
            dp3[i+1][j+c] = dp3[i+1][j+c].max(dp3[i][j] + a);
        } 
    }

    // println!("dp1: {:?}", dp1[dp1.len()-1]);
    // println!("dp2: {:?}", dp2[dp2.len()-1]);
    // println!("dp3: {:?}", dp3[dp3.len()-1]);

    let (mut m1, mut m2, mut m3) = (0, 0, 0);
    let (mut x1, mut x2, mut x3) = (0, 0, 0);
    let mut end_flg = false;
    while x1 + x2 + x3 < x && !end_flg {
        if m1 <= m2 && m1 < m3 {
            for xi in (x1+1)..=x {
                if dp1[dp1.len()-1][xi] > 0 {
                    m1 = dp1[dp1.len()-1][xi];
                    x1 = xi;
                    break;
                }
                if xi == x {
                    end_flg = true;
                }
            }
        } else if m2 <= m1 && m2 < m3 {
            for xi in (x2+1)..=x {
                if dp2[dp2.len()-1][xi] > 0 {
                    m2 = dp2[dp2.len()-1][xi];
                    x2 = xi;
                    break;
                }
                if xi == x {
                    end_flg = true;
                }
            }
        } else {
            for xi in (x3+1)..=x {
                if dp3[dp3.len()-1][xi] > 0 {
                    m3 = dp3[dp3.len()-1][xi];
                    x3 = xi;
                    break;
                }
                if xi == x {
                    end_flg = true;
                }
            }
        }

        // println!("m: {:?}, x: {:?}", (m1, m2, m3), (x1, x2, x3));
        if x1 + x2 + x3 >= x { break; }
    }

    println!("{}", m1.min(m2).min(m3));

}