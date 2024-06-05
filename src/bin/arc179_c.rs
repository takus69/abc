use proconio::input_interactive;

fn add(i: usize, j: usize) -> usize {
    println!("+ {} {}", i, j);
    input_interactive! {
        p: usize,
    }
    p
}

fn compare(i: usize, j: usize) -> bool {
    println!("? {} {}", i, j);
    input_interactive! {
        q: usize,
    }
    if q == 1 { return true; }
    false
}

fn merge_sort(data: &mut [usize]) {
    // println!("merge sort data: {:?}", data);
    if data.len() <= 1 {
        return;
    }
    let length = data.len();
    let middle = length / 2;
    merge_sort(&mut data[..middle]);
    merge_sort(&mut data[middle..]);

    let mut work_vec: Vec<usize> = Vec::with_capacity(length);
    for i in 0..length {
        work_vec.push(data[i]);
    }
    work_vec[middle..].reverse();

    let mut l = 0;
    let mut r = length - 1;
    for i in 0..length {
        // println!("i: {}, data: {:?}, (l, r): ({}, {})=({}, {})", i, data, l, r, work_vec[l], work_vec[r]);
        if l == r {
            data[i] = work_vec[r];
            continue;
        }
        // println!("compre r:l = {} {}", work_vec[r], work_vec[l]);
        if compare(work_vec[r], work_vec[l]) {
        // if work_vec[l] > work_vec[r] {
            // println!("i: {}, r: {} {}", i, r, work_vec[r]);
            data[i] = work_vec[r];
            r -= 1;
        } else {
            // println!("i: {}, l: {} {}", i, l, work_vec[l]);
            data[i] = work_vec[l];
            l += 1;
        }
    }
    // println!("merge sort before data: {:?}", data);
}

fn binary_search(vec: &[usize], target: usize) -> usize {
    let mut ng = -1;
    let mut ok = vec.len() as i64;

    while ng + 1 < ok {
        // println!("ng: {}, ok: {}", ng, ok);
        let mid = (ng + ok) / 2;
        // if vec[mid] < target {
        if compare(vec[mid as usize], target) {
            ng = mid;
        } else {
            ok = mid;
        }
    }
    ok as usize
}

fn main() {
    input_interactive! {
        n: usize,
    }

    let mut data: Vec<usize> = (1..=n).collect();
    // println!("before sort: {:?}", data);
    merge_sort(&mut data);
    // println!("sorted: {:?}", data);

    // 順に最小と最大を足していく
    while data.len() > 1 {
        // let next = data[0] + data[data.len()-1];
        let next = add(data[0], data[data.len()-1]);
        data.remove(0);
        data.remove(data.len()-1);
        // println!("before insert: {:?}", data);
        let i = binary_search(&data, next);
        data.insert(i, next);
        // println!("data: {:?}, i: {}, next: {}", data, i, next);
    }
    println!("!");
}