fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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
    let v1 = read_vec::<u64>();
    let _n = v1[0];
    let m = v1[1];

    let mut v2 = read_vec::<u64>();
    let vlen = v2.len();
    v2.sort();

    let mut v3 = Vec::new();
    for _ in 0..m {
        let v4 = read_vec::<u64>();
        let bj = v4[0];
        let cj = v4[1];
        v3.push((bj, cj));
    }

    v3.sort_by_key(|&(_, c)| c);
    v3.reverse();

    let mut len: usize = 0;
    for (b, c) in v3 {
        if v2[len] < c {
            for j in 0..(b as usize) {
                let lj = len + j;
                if lj < vlen && v2[lj] < c {
                    v2[lj] = c;
                } else {
                    break;
                }
            }
        } else {
            break;
        }

        len += b as usize;
        if len > vlen - 1 {
            break;
        }
    }

    let s = v2.iter().sum::<u64>();
    println!("{}", s);
}
