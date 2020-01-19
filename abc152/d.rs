fn keta(x: usize) -> usize {
    let mut keta = 0;
    let mut y = x;
    while y > 0 {
        keta += 1;
        y /= 10;
    }
    keta
}

fn top(x: usize) -> usize {
    let mut top = 0;
    let mut y = x;
    while y > 0 {
        top = y;
        y /= 10;
    }
    top
}

fn main() {
    let n = read::<usize>();
    let k = keta(n);
    //let t = top(n);
    //let b = n % 10;

    let mut count = 0;
    for i in 1..n + 1 {
        if i % 10 == 0 {
            continue;
        }
        for j in 1..k + 1 {
            //println!("{} {}", i, j);
            if j == 1 {
                if top(i) == i % 10 {
                    count += 1;
                }
            } else if j == 2 {
                if top(i) + i % 10 * 10 <= n {
                    count += 1;
                }
            } else if j == k {
                if i % 10 < top(n) {
                    count += 10usize.pow((j - 2) as u32);
                } else if i % 10 == top(n) {
                    let a = 10usize.pow((k - 1) as u32) * (i % 10);
                    count += (n - a) / 10;
                    if top(i) <= n % 10 {
                        count += 1;
                    }
                }
            } else {
                count += 10usize.pow((j - 2) as u32);
            }
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
