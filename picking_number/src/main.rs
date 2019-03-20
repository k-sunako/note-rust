use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    // 標準入力から全部bufに読み込む
    std::io::stdin().read_to_string(&mut buf).unwrap();

    // 読み込んだStringを空白で分解する
    let mut iter = buf.split_whitespace();

    let N: usize = iter.next().unwrap().parse().unwrap();
    // let q: usize = iter.next().unwrap().parse().unwrap();

    let numbers: Vec<usize> = iter.map(|x| x.parse().unwrap()).collect();

    let mut m: HashMap<usize,usize> = HashMap::new();
    for n in &numbers {
        let e = m.entry(n).or_insert(0);
        *e += 1;
    }

    let mut max = -1;
    for (k, v) in m {
        let aaa = k;
        if m.contains_key(aaa-1) {
            if max < v + *m.entry(k-1) {
                max = v + *m.entry(k-1)
            }
        }
        if m.contains_key(k+1) {
            
        }
    }
}
