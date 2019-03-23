use std::io::{self, BufRead};

// https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem

fn main() {
    // 標準入力からの行単位での取得
    // https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let mut line = iterator.next().unwrap().unwrap();
    let n: u32 = line.parse().unwrap();

    line = iterator.next().unwrap().unwrap();
    let scores: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    line = iterator.next().unwrap().unwrap();
    let m: u32 = line.parse().unwrap();

    line = iterator.next().unwrap().unwrap();
    let alice: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", n);
    println!("{:?}", scores);
    println!("{:?}", m);
    println!("{:?}", alice);
}

// 読み込んだStringを空白で分解する
//    let mut iter = buf.split_whitespace();

//  let _N: usize = iter.next().unwrap().parse().unwrap();
// let q: usize = iter.next().unwrap().parse().unwrap();
