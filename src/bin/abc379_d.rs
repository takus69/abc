use proconio::input;

fn main() {
    input! {
        q_cnt: usize,
    }
    let mut t: Vec<usize> = vec![0];
    let mut h: Vec<usize> = vec![0];
    let mut x: Vec<usize> = vec![0];
    let mut index = 0;
    for i in 0..q_cnt {
        input! {
            qi: usize,
        }
        if qi == 1 {
            x[index] += 1;
        } else if qi == 2 {
            input! {
                ti: usize,
            }
            t[index] = ti;
            x.push(0);
            t.push(0);
            h.push(0);
            index += 1;
        } else if qi == 3 {
            input! {
                hi: usize,
            }
            h[index] = hi;
            x.push(0);
            t.push(0);
            h.push(0);
            index += 1;
        }
    }
    h.push(0);
    x.push(0);
    x.reverse();
    t.reverse();
    h.reverse();
    
    let mut sum_t = vec![0];
    let mut pre = 0;
    for &ti in t.iter() {
        pre += ti;
        sum_t.push(pre);
    }

    // println!("x: {}, {:?}", x.len(), x);
    // println!("t: {}, {:?}", t.len(), t);
    // println!("h: {}, {:?}", h.len(), h);
    // println!("sum_t: {}, {:?}", sum_t.len(), sum_t);
    let mut last_i = x.len()-1;
    for i in (0..x.len()).rev() {
        let hi = h[i];
        if hi > 0 {
            let mut ans = 0;
            while sum_t[last_i] - sum_t[i] >= hi {
                ans += x.pop().unwrap();
                last_i -= 1;
            }
            println!("{}", ans);
        }
    }

}