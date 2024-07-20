use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut p1: [Chars; 4],
        mut p2: [Chars; 4],
        mut p3: [Chars; 4],
    }
    // ポリオミノを左上に寄せて、ポリオミノを含む長方形のみを抜き出す
    fn poly(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut ret = vec![vec!['.'; 4]; 4];
        let mut min_i = 4;
        let mut min_j = 4;
        for i in 0..4 {
            for j in 0..4 {
                if p[i][j] == '#' {
                    min_i = min_i.min(i);
                    min_j = min_j.min(j);
                    break;
                }
            }
        }
        let mut max_i = 0;
        let mut max_j = 0;
        for i in min_i..4 {
            for j in min_j..4 {
                if p[i][j] == '#' {
                    ret[i-min_i][j-min_j] = '#';
                    max_i = max_i.max(i-min_i);
                    max_j = max_j.max(j-min_j);
                }
            }
        }

        let ret = ret[0..=max_i].to_vec();
        let mut ret2: Vec<Vec<char>> = Vec::new();
        for r in ret.iter() {
            ret2.push(r[0..=max_j].to_vec());
        }
        ret2
    }

    // 並行移動させる
    fn r#move(p: &Vec<Vec<char>>, h: usize, w: usize) -> Option<Vec<Vec<char>>> {
        let mut ret = vec![vec!['.'; 4]; 4];
        for i in 0..p.len() {
            for j in 0..p[0].len() {
                if p[i][j] == '#' {
                    if i+h < 4 && j+w < 4 {
                        ret[i+h][j+w] = '#';
                    } else {
                        return None
                    }
                }
            }
        }

        Some(ret)
    }

    // 回転させる
    fn rotate(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        // pを時計回りに90*r度回転させる
        let mut ret = vec![vec!['.'; 4]; 4];
        let mut max_i = p.len()-1;
        let mut max_j = p[0].len()-1;
        for i in 0..=max_i {
            for j in 0..=max_j {
                let mut i2 = j;
                let mut j2 = max_i - i;
                ret[i2][j2] = p[i][j];

            }
        }
        let mut max_i = 0;
        let mut max_j = 0;
        for i in 0..4 {
            for j in 0..4 {
                if ret[i][j] == '#' {
                    max_i = max_i.max(i);
                    max_j = max_j.max(j);
                }
            }
        }
        let ret = ret[0..=max_i].to_vec();
        let mut ret2: Vec<Vec<char>> = Vec::new();
        for r in ret.iter() {
            ret2.push(r[0..=max_j].to_vec());
        }
        ret2
    }

    // 全移動する
    fn all_move(p: &Vec<Vec<char>>, target: &mut Vec<Vec<Vec<char>>>) {
        for i in 0..=(4-p.len()) {
            for j in 0..=(4-p[0].len()) {
                if let Some(p) = r#move(&p, i, j) {
                    target.push(p);
                }
            }
        }
    }

    // 1個目は並行移動のみ全て試す
    let mut target1: Vec<Vec<Vec<char>>> = Vec::new();
    let p1 = poly(&p1);
    all_move(&p1, &mut target1);
    // 2個目, 3個目は並行移動と回転(4回)を全て試す
    let mut target2: Vec<Vec<Vec<char>>> = Vec::new();
    let p2 = poly(&p2);
    all_move(&p2, &mut target2);
    let p2 = rotate(&p2);
    all_move(&p2, &mut target2);
    let p2 = rotate(&p2);
    all_move(&p2, &mut target2);
    let p2 = rotate(&p2);
    all_move(&p2, &mut target2);

    let mut target3: Vec<Vec<Vec<char>>> = Vec::new();
    let p3 = poly(&p3);
    all_move(&p3, &mut target3);
    let p3 = rotate(&p3);
    all_move(&p3, &mut target3);
    let p3 = rotate(&p3);
    all_move(&p3, &mut target3);
    let p3 = rotate(&p3);
    all_move(&p3, &mut target3);
    // println!("target1: {}, target2: {}, target3: {}", target1.len(), target2.len(), target3.len());

    for t1 in target1.iter() {
        for t2 in target2.iter() {
            let mut ret2 = t1.clone();
            let mut flg = true;
            for i in 0..4 {
                for j in 0..4 {
                    if t2[i][j] == '#' {
                        if ret2[i][j] == '#' {
                            flg = false;
                            break;
                        }
                        ret2[i][j] = '#';
                    }
                }
                if !flg {
                    break;
                }
            }
            if !flg { continue; }
            for t3 in target3.iter() {
                let mut ret3 = ret2.clone();
                let mut flg = true;
                for i in 0..4 {
                    for j in 0..4 {
                        if t3[i][j] == '#' {
                            if ret3[i][j] == '#' {
                                flg = false;
                                break;
                            }
                            ret3[i][j] = '#';
                        }
                    }
                    if !flg { break; }
                }
                if !flg { continue; }
                let mut flg = true;
                for i in 0..4 {
                    for j in 0..4 {
                        if ret3[i][j] != '#' {
                            flg = false;
                            break;   
                        }
                    }
                    if !flg { break; }
                }
                if flg {
                    println!("Yes");
                    std::process::exit(0);
                }
            }
        }
    }
    println!("No");
}