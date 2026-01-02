use proconio::input;
use itertools::Itertools;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    score: usize,
    ans: Vec<char>,
    a: Vec<isize>,
}

impl State {
    fn new() -> Self {
        let score = 0;
        let ans: Vec<char> = Vec::new();
        let a: Vec<isize> = vec![0; 20];

        Self { score, ans, a }
    }

    fn cmd(&mut self, c: char, p: usize, q: usize, r: usize) {
        self.score += self.simulate(c, p, q, r);

        let d = if c == 'A' { 1 } else { -1 };
        for i in [p, q, r] {
            self.a[i] += d;
        }
        self.ans.push(c);
    }

    fn simulate(&self, c: char, p: usize, q: usize, r: usize) -> usize {
        let mut add_score = 0;
        let d = if c == 'A' { 1 } else { -1 };
        for i in [p, q, r] {
            if self.a[i] == -d {
                add_score += 1;
            }
        }
        for i in 0..20 {
            if self.a[i] == 0 && ![p,q,r].contains(&i) {
                add_score += 1;
            }
        }

        add_score
    }
}

fn main() {
    input! {
        t: usize,
        mut pqr: [(usize, usize, usize); t],
    }
    for i in 0..t {
        pqr[i].0 -= 1;
        pqr[i].1 -= 1;
        pqr[i].2 -= 1;
    }

    let mut beams: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let beam_width = 1000;

    let state = State::new();
    beams.push(Reverse(state));

    for &(p, q, r) in pqr.iter() {
        let mut next_beams: BinaryHeap<Reverse<State>> = BinaryHeap::new();
        while let Some(Reverse(state)) = beams.pop() {
            for c in ['A', 'B'] {
                let mut next_state = state.clone();
                next_state.cmd(c, p, q, r);
                next_beams.push(Reverse(next_state));
                if next_beams.len() > beam_width {
                    next_beams.pop();
                }
            }
        }
        beams = next_beams;
    }

    let Reverse(state) = beams.iter().last().unwrap();

    eprintln!("score: {}", state.score);
    println!("{}", state.ans.iter().join(" "));
}