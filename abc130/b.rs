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
    //let n = read::<u32>();
    let v = read_vec::<u32>();
    let n = v[0];
    let x = v[1];

    let ls = read_vec::<u32>();

    let mut d = 0;
    for i in 0..n {
        d = d + ls[i as usize];
        if d > x {
            println!("{}", i + 1);
            break;
        }
    }
    if d <= x {
        println!("{}", n + 1);
    }
}
