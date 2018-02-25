fn foo() -> i32 {
    let x = 3;
    let y = 4;
    let z = x + y;
    z
}

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

    let mut s = "hello, world".to_owned();
    {
        let k = &mut s[3..6];
        k.make_ascii_uppercase();
    }
    println!("{}", s);

    let v = vec!["hello,", "world!", "yay"];
    println!("{}", v.join(" "));

    println!("{}", "th\u{e9}");
    println!("{}", "the\u{301}");

    println!("{}", foo());
}
