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

fn cango(t: i32, x: i32, y: i32) -> bool {
    let xab = if x >= 0 { x } else { -x };
    let yab = if y >= 0 { y } else { -y };
    if xab + yab > t {
        return false;
    }

    (xab + yab) % 2 == t % 2
}

fn main() {
    let n = read::<i32>();
    let mut v = vec![];
    for _ in 0..n {
        v.push(read_vec::<i32>());
    }

    let mut vs: Vec<(i32, i32, i32)> = vec![];
    for (i, vi) in v.iter().enumerate() {
        if i == 0 {
            vs.push((vi[0], vi[1], vi[2]));
        } else {
            vs.push((
                vi[0] - v[i - 1][0],
                vi[1] - v[i - 1][1],
                vi[2] - v[i - 1][2],
            ));
        }
    }

    let mut retb = true;
    for vi in vs {
        if !cango(vi.0, vi.1, vi.2) {
            retb = false;
            break;
        }
    }
    if retb {
        println!("Yes");
    } else {
        println!("No");
    }
}
