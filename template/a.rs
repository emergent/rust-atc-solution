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

fn main() {
    /* single input
    let n = read::<usize>();
    let n = read::<u32>();
    */

    /* vec input
    let v = read_vec::<u32>();
    let a = v[0];
    let b = v[1];
    let c = v[2];
    */

    /* loop pattern
    let mut count = 0;
    loop {
        if  { break; }
        count += 1;
    }
    println!("{}", count);
    */

    /* yes/no pattern
    yn(a == b);
    */
}
