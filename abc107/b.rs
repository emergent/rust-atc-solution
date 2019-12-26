fn main() {
    let v = read_vec::<usize>();
    let h = v[0];
    let w = v[1];
    let mut v = vec![];
    let mut ansv: Vec<Vec<char>> = vec![];

    for _ in 0..h {
        let line = read::<String>().chars().collect::<Vec<_>>();
        if !line.iter().all(|&c| c == '.') {
            v.push(line);
            ansv.push(vec![]);
        }
    }

    let newh = v.len();
    for j in 0..w {
        let mut f = false;
        for i in 0..newh {
            if v[i][j] != '.' {
                f = true;
                break;
            }
        }
        if f {
            for i in 0..newh {
                ansv[i].push(v[i][j]);
            }
        }
    }
    for i in 0..newh {
        for vv in ansv[i].clone() {
            print!("{}", vv);
        }
        println!("");
    }
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
