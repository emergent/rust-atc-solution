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
    let hs = read_vec::<u32>();

    let mut max_count = 0;
    let mut tmp_count = 0;
    for i in 0..n - 1 {
        if hs[i + 1] <= hs[i] {
            tmp_count += 1;
        } else {
            if tmp_count >= max_count {
                max_count = tmp_count;
            }
            tmp_count = 0;
        }
    }

    if tmp_count >= max_count {
        max_count = tmp_count;
    }

    println!("{}", max_count);
}
