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

fn f(a: u32) -> u32 {
    if a % 2 == 0 {
        a / 2
    } else {
        3 * a + 1
    }
}

fn main() {
    let s = read::<u32>();

    let mut v = vec![];
    let mut m = 1;
    let mut a = s;
    loop {
        if v.contains(&a) {
            break;
        } else {
            v.push(a);
        }

        a = f(a);
        m += 1;
    }
    println!("{}", m);
}
