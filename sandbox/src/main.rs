fn select_ref<'a, 'b, 'c>(a: &'a i32, b: &'b i32, selector: i32) -> &'c i32 where 'a: 'c, 'b: 'c {
    if selector == 0 {
        a
    } else {
        b
    }
}

#[test]
fn test_select_ref() {
    let a = 42;
    let b = 36;
    assert_eq!(*select_ref(&a, &b, 0), a);
}

struct S<'a> {
    r: &'a i32,
}

#[test]
fn test_struct_ref() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10);
}

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

static mut STATIC_REF: &i32 = &400;

unsafe fn set_static(p: &'static i32) {
    STATIC_REF = p;
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

    static STATIC: i32 = 42;
    unsafe {
        set_static(&STATIC);
        println!("STATIC is {}", STATIC_REF);
    }
}
