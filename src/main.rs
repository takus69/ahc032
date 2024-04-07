use proconio::input;
use rand::Rng;
use rand::seq::SliceRandom;

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

#[derive(Debug, Clone)]
struct Stamp {
    s: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Answer {
    board: Board,
    stamps: Vec<Stamp>,
    m: Vec<usize>,
    poses: Vec<Pos>,
}

impl Answer {
    fn new(board: Board, stamps: Vec<Stamp>) -> Self {
        let m: Vec<usize> = Vec::new();
        let poses: Vec<Pos> = Vec::new();
        Answer { board, stamps, m, poses }
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
        self.board.stamp(pos, &self.stamps[m]);
    }

    fn best(&mut self) -> bool {
        let n = self.board.n;
        let mut max_diff = 0;
        let mut max_mi = 0;
        let mut max_pos = Pos{ i: 0, j: 0 };
        for mi in 0..self.stamps.len() {
            for i in 0..(n-2) {
                for j in 0..(n-2) {
                    let pos = Pos{ i, j };
                    let diff = self.board.trial(&pos, &self.stamps[mi]);
                    if max_diff < diff {
                        max_diff = diff;
                        max_mi = mi;
                        max_pos = pos;
                    }
                }
            }
        }
        if max_diff > 0 {
            self.add(max_mi, &max_pos);
            true
        } else {
            false
        }
    }

    fn rand(&mut self) -> Answer {
        let mut ans = self.clone();
        let mut rng = rand::thread_rng();
        let mi = rng.gen_range(0..self.stamps.len()); 
        let i = rng.gen_range(0..(self.board.n-2));
        let j = rng.gen_range(0..(self.board.n-2));
        ans.add(mi, &Pos{ i, j });
        ans
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
    let board = Board { n, a: a.clone() };
    let mut stamps: Vec<Stamp> = Vec::new();
    for i in 0..m {
        stamps.push(Stamp { s: s[i].clone() });
    }
    let mut ans = Answer::new(board.clone(), stamps.clone());

    // 候補作成
    let trial = 500;
    let mut candidate: Vec<(Answer, usize)> = Vec::new();
    for _ in 0..(trial*2-1) {
        let ans2 = ans.rand();
        let score = ans2.board.score();
        candidate.push((ans2, score));
    }
    ans.best();
    candidate.push((ans.clone(), ans.board.score()));
    // println!("len of candidate: {}", candidate.len());
    for _ in 1..k {
        let mut tmp: Vec<(Answer, usize)> = Vec::new();
        for (mut ans, score) in candidate {
            // ランダムで1個登録
            let ans2 = ans.rand();
            let score = ans2.board.score();
            tmp.push((ans2, score));
            // 一番いい結果を登録
            ans.best();
            tmp.push((ans.clone(), ans.board.score()));
        }
        // ソートして上位trial個とランダムtrail個取得する
        tmp.sort_by_key(|&(_, score)| score);
        // println!("tmp: {} {:?}", tmp.len(), tmp.iter().map(|&(_, score)| score).collect::<Vec<_>>());
        candidate = tmp.drain((tmp.len()-trial)..tmp.len()).collect();
        // println!("candidate: {} {:?}", candidate.len(), candidate.iter().map(|&(_, score)| score).collect::<Vec<_>>());
        let mut rng = rand::thread_rng();
        let randoms: Vec<(Answer, usize)> = tmp.choose_multiple(&mut rng, trial).cloned().collect();
        candidate.extend(randoms);
        // println!("candidate: {} {:?}", candidate.len(), candidate.iter().map(|&(_, score)| score).collect::<Vec<_>>());
    }
    candidate.sort_by_key(|&(_, score)| score);
    let (ans, score) = &candidate[candidate.len()-1];
    let mut ans = ans.clone();
    for _ in 0..(k-ans.m.len()) {
        let flg = ans.best();
        if !flg { break; }
    }
    ans.ans();

    eprintln!("{{ \"score\": {} }}", ans.board.score());
}
