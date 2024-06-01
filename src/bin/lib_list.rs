use abc::*;
use std::collections::HashMap;

fn main() {
    // エラトステネスの篩
    let p = primes(100);
    println!("100までの素数のリストは、{:?}です", p);

    // 繰り返し二乗法
    let ret = pow(3, 5);
    println!("3の5乗は、{}です。繰り返し二乗法で大きなnでも早く計算できます", ret);
    
    // modint
    let ret = modint(3, 1<<60, 107);
    println!("3の{}乗を107で割った余りは、{}です。繰り返し二乗法でものすごい大きい指数計算を早くできます", 1i64<<60, ret);

    // UnionFind
    let mut uf = UnionFind::new(5);
    uf.unite(0, 1);
    uf.unite(1, 2);
    println!("2は、{}グループで、グループの要素数は、{}です。", uf.root(2), uf.size(2));
    println!("1と2は同じグループですか？: {}", uf.same(1, 2));

    // BFS(幅優先探索)と経路復元
    let v = vec![1, 3, 5, 7, 9];
    let mut e = HashMap::new();
    e.insert(1, vec![3, 9]);
    e.insert(3, vec![5, 1, 9]);
    e.insert(5, vec![7]);
    e.insert(7, vec![]);
    e.insert(9, vec![1, 5]);
    let start = 1;
    let end = 7;
    let (path, prev) = bfs(1, v, e);
    println!("{}から{}までの距離は、{}です", start, end, path[&end]);
    if let Some(path2) = reconstruct_path(end, path.clone(), prev.clone()) {
        println!("{}から{}までの最短経路の一つは、{:?}です", start, end, path2);
    }
    let end = 9;
    println!("{}から{}までの距離は、{}です", start, end, path[&end]);
    if let Some(path2) = reconstruct_path(end, path, prev) {
        println!("{}から{}までの最短経路の一つは、{:?}です", start, end, path2);
    }

    // bit全探索
    let cnt = bit_manipulation(4);
    println!("4要素のうち使用有無のパターンで要素がつかわれる数は、{}です", cnt);

}