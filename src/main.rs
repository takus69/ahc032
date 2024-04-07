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
    let diff = board.stamp(&Pos{ i: 0, j: 0 }, &stamps[0]);
    // ans.add(0, &Pos{ i: 0, j: 0 });
    ans.ans();

    eprintln!("{{ \"score\": {} }}", board.score());
}
