fn foo() -> i32 {
    let x = 3;
    let y = 4;
    let z = x + y;
    z
}

fn bar(v: &Vec<i32>) {
    println!("{:?}", v);
}

use std::collections::HashMap;

fn print_map(m: &HashMap<i32, String>) {
    for (i, s) in m {
        println!("'{}' at [{}]", s, i);
    }
}

#[macro_use]
extern crate maplit;

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

    let w = vec![1, 2, 3];
    for _ in 1..10 {
        bar(&w);
    }

    let map = hashmap![
        1 => "one".to_string(),
        42 => "forty two".to_string(),
    ];
    print_map(&map);
    assert_eq!(map[&42], "forty two");

    let i: i32 = 400;
    let p = &i;
    let pp = &p;
    println!("{}", pp);

    let qq = &&i;
    assert_eq!(pp, qq);
}
