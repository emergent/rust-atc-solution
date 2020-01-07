fn main() {
    let n = read::<usize>();
    let mut v = read_vec::<i64>();
    for i in 0..n {
        v[i] = v[i] - (i + 1) as i64;
    }
    v.sort();
    let med1 = v[n / 2];
    let med2 = if n > 1 { v[n / 2 + 1] } else { v[0] };

    //println!("{:?}", v);

    let mut ans = 1000000000000000000;
    for b in vec![med1, med2] {
        let mut sad = 0;
        for i in 0..n {
            sad += (v[i] - b).abs();
        }
        //println!("{}: {}", b, sad);
        ans = std::cmp::min(ans, sad);
    }
    println!("{}", ans);
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
