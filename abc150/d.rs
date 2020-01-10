fn main() {
    let v = read_vec::<u64>();
    let n = v[0];
    let m = v[1];
    let a = read_vec::<u64>();

    let a_half = a.into_iter().map(|ai| ai / 2).collect::<Vec<_>>();
    let mut a_gcd = a_half[0];
    for i in 1..n as usize {
        a_gcd = gcd(a_gcd, a_half[i]);
    }

    let a2 = a_half.into_iter().map(|ai| ai / a_gcd).collect::<Vec<_>>();
    let mut a_lcm = 1;
    for ai in a2 {
        if ai % 2 == 0 {
            println!("0");
            return;
        } else {
            a_lcm = lcm(a_lcm, ai);
        }
    }
    let aa = a_gcd * a_lcm;
    let ans = if (m / aa) % 2 == 0 {
        m / (aa * 2)
    } else {
        m / (aa * 2) + 1
    };
    println!("{}", ans);
}

fn gcd(x: u64, y: u64) -> u64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
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
