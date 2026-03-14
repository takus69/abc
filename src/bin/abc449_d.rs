use proconio::input;

fn main() {
    input! {
        l: isize,
        r: isize,
        d: isize,
        u: isize,
    }
    let mut ans = 0;
    for x in l..=r {
        let abs_x = x.abs();
        if abs_x < d {
            ans += (u-d+1)/2;
            if (u-d+1)%2==1 && d%2==0 { ans += 1; }
            // println!("1: x: {}, ans: {}", x, ans);
        } else if -abs_x <= d && abs_x < u {
            if x%2 == 0 {
                ans += abs_x-d+1;
            }
            ans += (u-abs_x)/2;
            if (u-abs_x)%2==1 && (abs_x+1)%2==0 { ans += 1; }
            // println!("2: x: {}, ans: {}", x, ans);
        } else if d < -abs_x && abs_x < u {
            if x%2 == 0 {
                ans += abs_x*2+1;
            }
            ans += (u-abs_x)/2;
            if (u-abs_x)%2==1 && (abs_x+1)%2==0 { ans += 1; }
            ans += (-abs_x-d)/2;
            if (-abs_x-d)%2==1 && d%2==0 { ans += 1; }
            // println!("6: x: {}, ans: {}", x, ans);
        } else if -abs_x <= d && u <= abs_x {
            if x%2 == 0 {
                ans += u-d+1;
            }
            // println!("3: x: {}, ans: {}", x, ans);
        } else if d < -abs_x && -abs_x <= u {
            if x%2 == 0 {
                ans += u+abs_x+1;
            }
            ans += (-abs_x-d)/2;
            if (-abs_x-d)%2==1 && d%2==0 { ans += 1; }
            // println!("4: x: {}, ans: {}", x, ans);
        } else {
            ans += (u-d+1)/2;
            if (u-d+1)%2==1 && d%2==0 { ans += 1; }
            // println!("5: x: {}, ans: {}", x, ans);
        }
    }
    println!("{}", ans);
}