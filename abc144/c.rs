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
    let n = read::<u64>();
    let mut root_n = 0;
    if n == 2 {
        println!("1");
        return;
    }

    for i in 1..n {
        if i * i > n {
            root_n = i;
            break;
        }
    }

    let mut min = 1000000000000;
    for i in 1..root_n + 1 {
        if n % i == 0 {
            min = std::cmp::min(min, i + (n / i) - 2);
        }
    }
    println!("{}", min);
}
