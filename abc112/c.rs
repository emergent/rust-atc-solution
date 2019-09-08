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
    let ps = read_vec2::<i64>(n as u32);
    let mut sample = 0;
    for (i, p) in ps.iter().enumerate() {
        if p[2] != 0 {
            sample = i;
            break;
        }
    }

    for x in 0..101 {
        for y in 0..101 {
            let h = ps[sample][2] + (ps[sample][0] - x).abs() + (ps[sample][1] - y).abs();
            let mut f = true;

            for p in &ps {
                let px = p[0];
                let py = p[1];
                let ph = p[2];

                if std::cmp::max(h - (px - x).abs() - (py - y).abs(), 0) != ph {
                    f = false;
                    break;
                }
            }
            if f {
                println!("{} {} {}", x, y, h);
                return;
            }
        }
    }
}
