fn main() {
    let v = read_vec::<usize>();
    let h = v[0];
    let w = v[1];

    let mut t = vec![vec!['.'; w + 2]; h + 2];
    for i in 0..h {
        let line = read::<String>().chars().collect::<Vec<_>>();
        for j in 0..w {
            t[i + 1][j + 1] = line[j];
        }
    }

    for i in 1..h + 1 {
        for j in 1..w + 1 {
            if t[i][j] == '#'
                && t[i - 1][j] == '.'
                && t[i + 1][j] == '.'
                && t[i][j - 1] == '.'
                && t[i][j + 1] == '.'
            {
                yn(false);
                return;
            }
        }
    }
    yn(true);
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
