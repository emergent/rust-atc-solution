fn main() {
    let s = read::<String>().chars().collect::<Vec<_>>();
    let t = read::<u32>();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut q = 0;
    for c in s {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => q += 1,
        }
    }

    let man = x.abs() + y.abs();
    if t == 1 {
        // max
        println!("{}", man + q);
    } else if t == 2 {
        // min
        if man >= q {
            println!("{}", man - q);
        } else {
            println!("{}", (q - man) % 2);
        }
    }
}

#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
#[allow(dead_code)]
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
#[allow(dead_code)]
fn yn(result: bool) {
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
