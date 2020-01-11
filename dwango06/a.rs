fn main() {
    let n = read::<usize>();
    let mut titles = vec![];
    let mut times = vec![];

    for _ in 0..n {
        let v = read_vec::<String>();
        let title = v[0].clone();
        let time = v[1].parse::<u32>().unwrap();
        titles.push(title);
        times.push(time);
    }
    let last_title = read::<String>();

    let mut f = false;
    let mut count = 0;
    for i in 0..n {
        if f {
            count += times[i];
        }
        if last_title == titles[i] {
            f = true;
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
