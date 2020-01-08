fn main() {
    let cs = read_vec2::<i32>(3);

    for i in 1..3 {
        let a1 = cs[i][2] - cs[i - 1][2];
        let a2 = cs[i][1] - cs[i - 1][1];
        let a3 = cs[i][0] - cs[i - 1][0];
        if a1 != a2 || a2 != a3 {
            yn(false);
            return;
        }

        let b1 = cs[2][i] - cs[2][i - 1];
        let b2 = cs[1][i] - cs[1][i - 1];
        let b3 = cs[0][i] - cs[0][i - 1];
        if b1 != b2 || b2 != b3 {
            yn(false);
            return;
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
