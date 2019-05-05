fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

#[allow(dead_code)]
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn calc(rest: u32, n: u32, satun: usize, satuv: &Vec<u32>, ans: (u32, u32, u32)) -> Option<(u32, u32, u32)> {
    let satu = satuv[satun];

    if satun == satuv.len() - 1 {
        if rest == satu * n {
            return Some((ans.0, ans.1, n));
        } else {
            return None;
        }
    }

    if rest > satu * n {
        return None
    }

    let rem = rest / satu;
    for i in (0..rem+1).rev() {
        if satu * i > rest || i > n { continue; }
        let ans_next = if satun == 0 { (i, ans.1, ans.2) } else { (ans.0, i, ans.2) };
        match calc(rest - satu * i, n - i, satun + 1, satuv, ans_next) {
            None => continue,
            Some(a) => return Some(a),
        }
    }

    None
}



fn main() {
    let v = read_vec::<u32>();
    let n: u32 = v[0];
    let y: u32 = v[1];

    let satuv = vec![10000, 5000, 1000];

    match calc(y, n, 0, &satuv, (0, 0, 0)) {
        None => println!("-1 -1 -1"),
        Some((i, j, k)) => println!("{} {} {}", i, j, k),
    }
}
