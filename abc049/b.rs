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

fn main() {
    let v = read_vec::<u32>();
    let h = v[0];
    let _w = v[1];

    for _i in 0..h {
        let s = read::<String>();
        /*
        let new_s = s.chars().map(|c| {
            let mut a = c.to_string();
            a.push(c);
            a
        }).collect::<String>();
        */
        println!("{}", s);
        println!("{}", s);
    }
}
