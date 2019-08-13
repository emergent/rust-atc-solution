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

fn main() {
    let v = read_vec::<u64>();
    let n = v[0] as u32;
    let mut x = v[1];

    let mut len = 2u64.pow(n);
    for i in 0..n {
        len += 2u64.pow(n - i - 1) * 3u64;
    }
    let mut pat = 2u64.pow(n + 1) - 1;

    let mut count: u64 = 0;
    loop {
        //println!("{} {}", len, pat);

        if x == (len + 1) / 2 {
            count += (pat + 1) / 2;
            break;
        } else if x == (len - 1) / 2 {
            count += (pat - 1) / 2;
            break;
        } else if x > (len + 1) / 2 {
            count += (pat + 1) / 2;
            x -= (len + 1) / 2;
        } else {
            x -= 1; // ban
        }

        pat = (pat - 1) / 2;
        len = (len - 3) / 2;

        if x == 0 {
            break;
        }
    }
    println!("{}", count);
}
