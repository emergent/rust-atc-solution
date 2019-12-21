fn main() {
    let n = read::<usize>();
    let s = read::<String>();
    if n % 2 == 0 {
        println!("-1");
        return;
    }
    let x = (n - 1) / 2;
    let mut ss = String::from("b");
    for i in 1..x + 1 {
        match i % 3 {
            1 => {
                ss.insert(0, 'a');
                ss.push('c');
            }
            2 => {
                ss.insert(0, 'c');
                ss.push('a');
            }
            _ => {
                ss.insert(0, 'b');
                ss.push('b');
            }
        }
    }
    if ss == s {
        println!("{}", x);
    } else {
        println!("-1");
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
