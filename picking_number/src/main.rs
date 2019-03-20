use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 読み込んだStringを空白で分解する
    let mut iter = buf.split_whitespace();

    let _N: usize = iter.next().unwrap().parse().unwrap();
    // let q: usize = iter.next().unwrap().parse().unwrap();

    let numbers: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();

    let mut m: HashMap<i32, i32> = HashMap::new();
    for n in &numbers {
        let x: i32 = *n;
        let e = m.entry(x).or_insert(0);
        *e += 1;
    }

    let mut max = -1;
    for (k, v) in &m {
        let next:i32 = *k + 1;

        if let Some(x) = m.get(&next)  {
            if max < v + x {
                max = v + x;
            }
        }

        if max < *v {
            max = *v;
        }
    }

    println!("{}", max)
}
