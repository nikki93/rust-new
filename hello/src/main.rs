fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(13, 13), 13);
    assert_eq!(gcd(37, 600), 1);
    assert_eq!(gcd(20, 100), 20);
    assert_eq!(gcd(624129, 2061517), 18913);
}

fn main() {
    let m = 25;
    let n = 60;
    println!("gcd of {} and {} is: {}", m, n, gcd(m, n));
}
