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

fn main() {
    let an = read::<i32>();
    let bn = read::<i32>();
    let cn = read::<i32>();
    let x  = read::<i32>();

    // note: cannot use `..=` and `step_by()` in Rust 1.15.1 @ AtCoder
    let mut pat = 0;
    for a in 0 .. (x/500)+1 {
        if a > an { break; }
        for b in 0 .. ((x-a)/100)+1 {
            if b > bn { break; }
            for c in 0 .. ((x-a-b)/50)+1 {
                if c > cn { break; }
                if (x - a*500 - b*100 - c*50) == 0 {
                    pat += 1;
                }
            }
        }
    }
    println!("{}", pat);
}
