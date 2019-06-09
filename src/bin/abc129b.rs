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

fn abs(x: i32) -> i32 {
    if x >= 0 {
        x
    } else {
        -x
    }
}

fn main() {
    let n = read::<i32>();
    let w = read_vec::<i32>();
    let mut min = 100000;

    for t in 0..n {
        let mut s1 = 0;
        let mut s2 = 0;
        for i in 0..t {
            s1 += w[i as usize];
        }
        for j in t..n {
            s2 += w[j as usize]
        }
        let sub = abs(s1 - s2);
        if sub < min {
            min = sub;
        }
    }
    println!("{}", min);
}
