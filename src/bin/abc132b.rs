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
    let ps = read_vec::<u32>();

    let mut count = 0;
    for i in 0..n - 2 {
        let mut sl = ps[i..i + 3].iter().map(|j| *j).collect::<Vec<u32>>();
        sl.sort();
        if sl[1] == ps[i + 1] {
            count += 1;
        }
    }
    println!("{}", count);
}
