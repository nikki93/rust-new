fn main() {
    const MAX: usize = 3000000;
    let mut sieve = [true; MAX];
    for i in 2..MAX {
        if sieve[i] {
            let mut j = i * i;
            while j < MAX {
                sieve[j] = false;
                j += i;
            }
        }
    }
    let n = 2000107;
    println!("{} is prime? {}", n, sieve[n]);
}
