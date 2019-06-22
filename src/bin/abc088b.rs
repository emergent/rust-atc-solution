fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let _n = read::<i32>();
    let mut an = read_vec::<i32>();

    an.sort();

    let mut alice = 0;
    let mut bob = 0;
    let mut ord = 0; // 0: alice, 1: bob
    loop {
        match an.pop() {
            Some(i) => match ord {
                0 => alice += i,
                _ => bob += i,
            },
            None => break,
        }
        if ord == 0 {
            ord = 1
        } else {
            ord = 0
        }
    }
    println!("{}", alice - bob);
}
