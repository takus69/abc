use proconio::input;

fn main() {
    input! {
        n: usize,
        udlr: [(usize, usize, usize, usize); n],
    }

    fn cumsum(map: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
        let mut ret = vec![vec![0; map[0].len()]; map.len()];
        for i in 0..map.len() {
            for j in 0..(map[0].len()-1) {
                ret[i][j+1] = ret[i][j] + map[i][j+1];
            }
        }
        for j in 0..map[0].len() {
            for i in 0..(map.len()-1) {
                ret[i+1][j] += ret[i][j];
            }
        }

        ret
    }

    let nn = 2000;

    let mut clouds: Vec<Vec<isize>> = vec![vec![0; nn+2]; nn+2]; 
    for &(u, d, l, r) in udlr.iter() {
        clouds[u][l] += 1;
        clouds[u][r+1] -= 1;
        clouds[d+1][l] -= 1;
        clouds[d+1][r+1] += 1;
    }
    let clouds = cumsum(&clouds);

    let mut cloud1: Vec<Vec<isize>> = vec![vec![0; nn+2]; nn+2];
    let mut cnt = 0;
    for i in 1..(clouds.len()-1) {
        for j in 1..(clouds[0].len()-1) {
            if clouds[i][j] == 1 {
                cloud1[i][j] += 1;
            }
            if clouds[i][j] == 0 {
                cnt += 1;
            }
        }
    }
    let cloud1 = cumsum(&cloud1);

    for &(u, d, l, r) in udlr.iter() {
        let ans = cloud1[d][r] - cloud1[d][l-1] - cloud1[u-1][r] + cloud1[u-1][l-1];
        println!("{}", cnt+ans);
    }
}