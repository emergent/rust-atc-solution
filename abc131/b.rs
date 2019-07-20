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

fn main() {
    let v = read_vec::<i32>();
    let n = v[0];
    let l = v[1];

    let mut tastes = Vec::new();
    for i in 1..n + 1 {
        tastes.push(l + i - 1);
    }

    let mut minabs = 10000000;
    let mut minj = 0;
    for j in tastes {
        if j.abs() < minabs {
            minabs = j.abs();
            minj = j;
        }
    }

    let sum_n = (l - 1) * n + (n * (n + 1) / 2);
    let sum_n_ = sum_n - minj;

    println!("{}", sum_n_);
}
