fn save_the_prisoner(n: u32, m: u32, s: u32) -> u32 {
    let t = m + (s - 1);
    if t % n == 0 {
        n
    } else {
        t % n
    }
}

#[test]
fn test_save_the_prisoner() {
    assert_eq!(save_the_prisoner(5, 2, 1), 2);
}

fn main() {
    println!("Hello, world!");
}
