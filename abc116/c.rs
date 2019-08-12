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
    let n = read::<usize>();
    let v = read_vec::<u32>();

    if v.len() == 1 {
        println!("{}", v[0]);
        return;
    }

    let mut count = 0;
    for i in 0..n {
        if i == 0 {
            if v[i] > 0 && v[i] >= v[i + 1] {
                count += v[i];
            }
        } else if i == n - 1 {
            if v[i] > v[i - 1] {
                count += v[i];
            }
        } else {
            if v[i] > v[i - 1] && v[i] >= v[i + 1] {
                count += v[i];
            } else if v[i] <= v[i - 1] && v[i] < v[i + 1] {
                count -= v[i];
            }
        }
    }
    println!("{}", count);
}
