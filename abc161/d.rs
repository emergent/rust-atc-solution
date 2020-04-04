use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let mut vd = VecDeque::<u64>::new();

    for i in 1..10 {
        vd.push_back(i);
    }

    let mut idx = 0;
    while let Some(val) = vd.pop_front() {
        idx += 1;
        if idx == n as u64 {
            println!("{}", val);
            break;
        }

        let x = val % 10;
        if x > 0 {
            vd.push_back(val * 10 + x - 1);
        }
        vd.push_back(val * 10 + x);
        if x < 9 {
            vd.push_back(val * 10 + x + 1);
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
