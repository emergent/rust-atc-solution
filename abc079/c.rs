fn main() {
    let v = read::<String>()
        .chars()
        .map(|c| c as i32 - 48)
        .collect::<Vec<_>>();
    for i in 0..1 << 3 {
        let mut a = v[0];
        for j in 0..3 {
            if (1 << j) & i == 0 {
                a -= v[j + 1];
            } else {
                a += v[j + 1];
            }
        }
        if a == 7 {
            let mut op = vec!['+'; 3];
            for j in 0..3 {
                if (1 << j) & i == 0 {
                    op[j] = '-';
                }
            }
            println!(
                "{}{}{}{}{}{}{}=7",
                v[0], op[0], v[1], op[1], v[2], op[2], v[3]
            );
            break;
        }
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
