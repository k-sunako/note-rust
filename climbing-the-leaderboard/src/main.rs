use std::io::{self, BufRead};

// https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem

fn main() {
    // 標準入力からの行単位での取得
    // https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let line = iterator.next().unwrap().unwrap();
    let _n: u32 = line.parse().unwrap();

    let line = iterator.next().unwrap().unwrap();
    let scores: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let line = iterator.next().unwrap().unwrap();
    let _m: u32 = line.parse().unwrap();

    let line = iterator.next().unwrap().unwrap();
    let alice: Vec<u32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let ret = climbing_leaderboard(scores, alice);
    println!("{:?}", ret);
    //println!("{:?}", n);
    //println!("{:?}", scores);
    //println!("{:?}", m);
    //println!("{:?}", alice);
}

#[test]
fn test_climbing_leaderboard() {
    assert_eq!(
        climbing_leaderboard(vec![100, 100, 50, 40, 40, 20, 10], vec![5, 25, 50, 120]),
        vec![6, 4, 2, 1]
    );
}

fn climbing_leaderboard(scores: Vec<u32>, alice: Vec<u32>) -> Vec<u32> {
    let mut ranks = vec![0; scores.len()];
    let mut rank = 0;
    for i in 0..scores.len() {
        if i == 0 {
            rank = 1;
        } else if scores[i - 1] == scores[i] {
            ;
        } else {
            rank += 1;
        }

        ranks[i] = rank;
    }

    let mut ret = vec![0; alice.len()];
    let mut i = scores.len() - 1;
    for j in 0..alice.len() {
        while i != 0 && scores[i] <= alice[j] {
            i -= 1;
        }

        if scores[i] <= alice[j] {
            ret[j] = ranks[i];
        } else {
            ret[j] = ranks[i] + 1;
        }
    }

    ret
}

// 読み込んだStringを空白で分解する
//    let mut iter = buf.split_whitespace();

//  let _N: usize = iter.next().unwrap().parse().unwrap();
// let q: usize = iter.next().unwrap().parse().unwrap();
