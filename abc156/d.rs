const MOD: usize = 1000000007;

fn repeat_sq(n: i64, p: i64, m: i64) -> i64 {
    if p == 0 {
        return 1;
    }

    if p % 2 == 0 {
        let t = repeat_sq(n, p / 2, m) % m;
        t * t % m
    } else {
        n * repeat_sq(n, p - 1, m) % m
    }
}

fn main() {
    let v = read_vec::<i64>();
    let n = v[0];
    let a = v[1];
    let b = v[2];
    let m = MOD as i64;

    let mut ans = repeat_sq(2, n, m);
    ans = (ans + m - 1) % m;

    let fac = |n, k, m| {
        let mut f = 1;
        for i in (1..n + 1).rev() {
            f *= i;
            f %= m;
            if i == n - k + 1 {
                break;
            }
        }
        f
    };

    let xa = fac(n, a, m);
    let aa = fac(a, a, m);
    let ya = repeat_sq(aa, m - 2, m);
    let nca = xa * ya % m;

    let xb = fac(n, b, m);
    let bb = fac(b, b, m);
    let yb = repeat_sq(bb, m - 2, m);
    let ncb = xb * yb % m;
    //println!("{} {} {} {} {} {} {}", ans, xa, xb, ya, yb, nca, ncb);

    ans = ((ans + m) - nca) % m;
    ans = ((ans + m) - ncb) % m;

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
