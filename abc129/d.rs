fn main() {
    let v = read_vec::<usize>();
    let h = v[0];
    let w = v[1];

    let mut r = vec![];
    let mut ansr = vec![vec![0u32; w]; h];

    for _ in 0..h {
        r.push(read::<String>().chars().collect::<Vec<_>>());
    }

    for i in 0..h {
        let mut count = 0;
        let mut last_j = 0;

        for j in 0..w {
            if r[i][j] == '.' {
                count += 1;
            } else if r[i][j] == '#' {
                for k in last_j..j {
                    ansr[i][k] += count;
                }
                count = 0;
                last_j = j + 1;
            }
        }
        for k in last_j..w {
            ansr[i][k] += count;
        }
    }

    //println!("{:?}", ansr);

    for j in 0..w {
        let mut count = 0;
        let mut last_i = 0;

        for i in 0..h {
            if r[i][j] == '.' {
                count += 1;
            } else if r[i][j] == '#' {
                for k in last_i..i {
                    ansr[k][j] += count;
                }
                count = 0;
                last_i = i + 1;
            }
        }
        for k in last_i..h {
            ansr[k][j] += count;
        }
    }

    //println!("{:?}", ansr);
    let mut a = 0;
    for i in 0..h {
        for j in 0..w {
            a = std::cmp::max(a, ansr[i][j]);
        }
    }
    println!("{}", a - 1);
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
