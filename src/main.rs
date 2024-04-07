use proconio::input;

const MOD: usize = 998244353;

#[derive(Debug, Clone)]
struct Board {
    n: usize,
    a: Vec<Vec<usize>>,
}

impl Board {
    fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                score += self.a[i][j];
            }
        }
        score
    }

    fn stamp(&mut self, pos: &Pos, stamp: &Stamp) -> i64 {
        let mut before = 0;
        let mut after = 0;
        for i in 0..3 {
            for j in 0..3 {
                before += self.a[pos.i+i][pos.j+j];
                self.a[pos.i+i][pos.j+j] += stamp.s[i][j];
                self.a[pos.i+i][pos.j+j] %= MOD;
                after += self.a[pos.i+i][pos.j+j];
            }
        }
        after as i64 - before as i64
    }

    fn trial(&self, pos: &Pos, stamp: &Stamp) -> i64 {
        let mut before = 0;
        let mut after = 0;
        for i in 0..3 {
            for j in 0..3 {
                before += self.a[pos.i+i][pos.j+j];
                let mut a = self.a[pos.i+i][pos.j+j] + stamp.s[i][j];
                a %= MOD;
                after += a;
            }
        }
        after as i64 - before as i64
    }

}

#[derive(Debug, Clone)]
struct Pos {
    i: usize,
    j: usize,
}

#[derive(Debug)]
struct Stamp {
    s: Vec<Vec<usize>>,
}

struct Answer {
    m: Vec<usize>,
    poses: Vec<Pos>,
}

impl Answer {
    fn new() -> Self {
        let m: Vec<usize> = Vec::new();
        let poses: Vec<Pos> = Vec::new();
        Answer { m, poses }
    }

    fn ans(&self) {
        println!("{}", self.m.len());
        for i in 0..self.m.len() {
            let pos = &self.poses[i];
            println!("{} {} {}", self.m[i], pos.i, pos.j);
        }
    }

    fn add(&mut self, m: usize, pos: &Pos) {
        self.m.push(m);
        self.poses.push(pos.clone());
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [[usize; n]; n],
        s: [[[usize; 3]; 3]; m],
    }
    let mut board = Board { n, a: a.clone() };
    let mut stamps: Vec<Stamp> = Vec::new();
    let mut ans = Answer::new();
    for i in 0..m {
        stamps.push(Stamp { s: s[i].clone() });
    }
    for _ in 0..k {
        let mut max_diff = 0;
        let mut max_mi = 0;
        let mut max_pos = Pos{ i: 0, j: 0 };
        for mi in 0..m {
            for i in 0..(n-2) {
                for j in 0..(n-2) {
                    let pos = Pos{ i, j };
                    let diff = board.trial(&pos, &stamps[mi]);
                    if max_diff < diff {
                        max_diff = diff;
                        max_mi = mi;
                        max_pos = pos;
                    }
                }
            }
        }
        // println!("{} {} {:?}", max_diff, max_mi, max_pos);
        if max_diff > 0 {
            board.stamp(&max_pos, &stamps[max_mi]);
            ans.add(max_mi, &max_pos);
        } else {
            break;
        }

    }
    ans.ans();

    eprintln!("{{ \"score\": {} }}", board.score());
}
