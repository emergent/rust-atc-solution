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
    let n = read::<usize>();
    let mut ls = read_vec::<u32>();
    ls.sort();

    let mut count = 0;

    let lsmax = ls[n - 1];
    for a in 0..n - 2 {
        if ls[a] > lsmax / 2 + 1 {
            let x = (n - a - 1) * (n - a - 2) / 2;
            count += x;
            //println!("a={}, x={}", a, x);
            continue;
        }
        for b in a + 1..n - 1 {
            let ab = ls[a] + ls[b];
            if ab > lsmax {
                let y = (n - b - 1) * (n - b) / 2;
                count += y;
                //println!("a={}, b={}, y={}", a, b, y);
                break;
            }

            for c in b + 1..n {
                if ls[c] < ab {
                    count += 1;
                //println!("a={}, b={}, c={}, +1", a, b, c);
                } else {
                    //println!("a={}, b={}, c={}, skip-c", a, b, c);
                    break;
                }
            }
        }
    }
    println!("{}", count);
}
