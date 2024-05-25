use proconio::input;
use ac_library::SccGraph;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut sg = SccGraph::new(n);
    for (a, b) in ab {
        sg.add_edge(a, b);
    }
    let ans = sg.scc();

    println!("{}", ans.len());
    for a in ans.iter() {
        println!("{} {}", a.len(), a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
    }
}