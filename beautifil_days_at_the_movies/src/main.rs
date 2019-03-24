fn beautiful_days(i: i32, j: i32, k: i32) -> i32 {
    let mut count = 0;

    for l in i..=j {
        let mut reverse = 0;
        let mut digit = (l as f32).log10() as i32;
        let mut tmp = l;
        while 0 <= digit {
            let mut pow = 1;
            for _m in 0..digit {
                pow *= 10;
            }

            reverse += pow * (tmp % 10);
            tmp /= 10;
            digit -= 1;
        }

        if (reverse - l).abs()  % k == 0 {
            count += 1;
        }
    }

    count
}


#[test]
fn test_beautiful_days() {
    assert_eq!(beautiful_days(20, 23, 6), 2);
}

fn main() {
    println!("Hello, world!");
}
