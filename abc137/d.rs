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

use std::collections::*;

fn main() {
    let a = read_vec::<usize>();
    let n = a[0];
    let m = a[1];

    let mut v = vec![];
    for _ in 0..n {
        let u = read_vec::<usize>();
        if u[0] <= m {
            v.push((m - u[0], u[1] as u64));
        }
    }
    v.sort_by_key(|&(a, _)| a);
    //println!("v={:?}", v);

    let mut ans = 0;
    let mut que = BinaryHeap::new();
    for j in 0..m + 1 {
        while let Some((a, b)) = v.pop() {
            if m - a > j {
                v.push((a, b));
                break;
            }
            que.push(b);
        }
        //println!("v={:?}, que={:?}", v, que);

        if let Some(b) = que.pop() {
            ans += b;
        }
    }
    println!("{}", ans);
}
