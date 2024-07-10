use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut grid = vec![vec!["."; w]; h];
    let DIR = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut i, mut j, mut d) = (0, 0, 0);
    for _ in 0..n {
        let g = grid[i][j];
        if g == "." {
            grid[i][j] = "#";
            d += 1;
            d %= 4;
        } else {
            grid[i][j] = ".";
            d += 3;
            d %= 4;
        }
        i += (h as i32 + DIR[d].0) as usize;
        i %= h;
        j += (w as i32 + DIR[d].1) as usize;
        j %= w;
    }
    for g in grid.iter() {
        println!("{}", g.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}