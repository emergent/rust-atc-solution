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
    let n = read::<usize>();
    let v = read_vec::<u64>();

    let mut a_even_sum = 0u64;
    for i in 0..n {
        if i % 2 == 1 {
            a_even_sum += v[i];
        }
    }
    let x1 = v.iter().fold(0u64, |sum, x| sum + x) - a_even_sum * 2;

    let mut v2 = Vec::new();
    v2.push(x1);

    let mut yama = (v[0] - (x1 / 2)) * 2;
    for i in 1..n {
        v2.push(yama);
        yama = (v[i] - (yama / 2)) * 2;
    }

    if v2.len() == n && yama == x1 {
        for j in 0..n {
            if j == n - 1 {
                println!("{}", v2[j]);
            } else {
                print!("{} ", v2[j]);
            }
        }
        return ();
    }
}
