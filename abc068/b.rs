fn main() {
    let n = read::<u32>();
    let mut max = 0;
    let mut maxi = 0;
    for i in 1..n + 1 {
        let mut a = i;
        let mut count = 0;
        while a > 0 && a % 2 == 0 {
            count += 1;
            a /= 2;
        }
        if count >= max {
            max = count;
            maxi = i;
        }
    }
    println!("{}", maxi);
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