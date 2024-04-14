use proconio::input;

#[derive(Clone)]
struct Board {
    a: Vec<Vec<i64>>,
    b: Vec<Vec<i64>>,
    used: Vec<Vec<bool>>,
    turn: i64,
}

impl Board {
    fn new(a: Vec<Vec<i64>>) -> Board {
        let b = vec![vec![0; 3]; 3];
        let turn = 1;
        let used = vec![vec![false; 3]; 3];
        Board { a, b, used, turn }
    }

    fn play(&mut self) -> i64 {
        // println!("b: {:?}", self.b);
        let mut winner = self.winner();
        if winner > 0 { return winner; }
        for i in 0..3 {
            for j in 0..3 {
                if self.used[i][j] { continue; }
                let mut board = self.clone();
                board.turn(i, j);
                winner = board.play();
                // println!("turn: {}, (i, j): ({}, {}), winner: {}, b: {:?}", self.turn, i, j, winner, board.b);
                if self.turn == winner {
                    // self.b = board.b;
                    // self.used = board.used;
                    // self.turn = board.turn;
                    break;
                }
            }
            if self.turn == winner {
                break;
            }
        }
        winner
    }

    fn turn(&mut self, i: usize, j: usize) {
        self.b[i][j] = self.turn;
        self.turn = (self.turn % 2) + 1;
        self.used[i][j] = true;
    }

    fn winner(&self) -> i64 {
        // 0: 引き分け、1,2: 勝ち, -1: 途中
        let check: Vec<Vec<(usize, usize)>> = vec![
            vec![(0, 0), (0, 1), (0, 2)],
            vec![(1, 0), (1, 1), (1, 2)],
            vec![(2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (1, 0), (2, 0)],
            vec![(0, 1), (1, 1), (2, 1)],
            vec![(0, 2), (1, 2), (2, 2)],
            vec![(0, 0), (1, 1), (2, 2)],
            vec![(0, 2), (1, 1), (2, 0)],
        ];

        let mut winner = 0;
        for i in 0..3 {
            for j in 0..3 {
                if !self.used[i][j] {
                    winner = -1;
                    break;
                }
            }
        }
        for c in check {
            if self.b[c[0].0][c[0].1] > 0 && self.b[c[0].0][c[0].1] == self.b[c[1].0][c[1].1] && self.b[c[1].0][c[1].1] == self.b[c[2].0][c[2].1] {
                winner = self.b[c[0].0][c[0].1];
            }
        }
        if winner == 0 {
            let mut a1 = 0;
            let mut a2 = 0;
            for i in 0..3 {
                for j in 0..3 {
                    if self.b[i][j] == 1 {
                        a1 += self.a[i][j];
                    } else {
                        a2 += self.a[i][j];
                    }
                }
            }
            if a1 > a2 {
                winner = 1;
            } else {
                winner = 2;
            }
        }
        // println!("turn: {}, winner: {}, b: {:?}, used: {:?}", self.turn, winner, self.b, self.used);
        winner
    }
}

fn main() {
    input! {
        a: [[i64; 3]; 3],
    }
    let mut board = Board::new(a);
    let winner = board.play();
    if winner == 1 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}