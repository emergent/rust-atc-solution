fn main() {
    let n = read::<usize>();
    let ps = read_vec::<u32>();
    let mut qs = vec![false; ps.len()];

    for (i, &p) in ps.iter().enumerate() {
        if i + 1 == p as usize {
            qs[i] = true;
        }
    }

    let mut count = 0;
    let mut i = 0;
    let len = qs.len();
    while i < len {
        if i < len - 1 {
            if qs[i] && qs[i + 1] {
                count += 1;
                i += 2;
            } else if qs[i] && !qs[i + 1] {
                count += 1;
                i += 2;
            } else {
                i += 1;
            }
        } else {
            if qs[i] {
                count += 1;
            }
            i += 1;
        }
    }

    println!("{}", count);
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
