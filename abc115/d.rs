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
    let n = v[0];
    let mut x = v[1];

    let mut len = 1;
    for _ in 0..n {
        len = 2u64 * len + 3u64;
    }
    let mut pat = 1;
    for _ in 0..n {
        pat = 2u64 * pat + 1u64;
    }

    let mut count: u64 = 0;
    loop {
        //println!("{} {} {} {}", len, pat, x, count);
        let mid = (len + 1) / 2;

        if x == 1 {
            if len == 1 {
                count += 1;
            }
            break;
        } else if x == mid - 1 {
            count += (pat - 1) / 2;
            break;
        } else if x == mid {
            count += (pat + 1) / 2;
            break;
        } else if x > mid {
            count += (pat + 1) / 2;
            x -= mid;
        } else {
            x -= 1; // ban
        }

        pat = (pat - 1) / 2;
        len = (len - 3) / 2;
    }
    println!("{}", count);
}
