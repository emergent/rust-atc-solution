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

fn f(ans: &mut Vec<u64>, e: &Vec<Vec<usize>>, x: usize) {
    let len = e[x].len();
    if len == 0 {
        ans[x] = 1;
    } else if len == 1 {
        f(ans, e, e[x][0]);
        ans[x] = 2 * ans[e[x][0]] + 1;
    } else {
        let mut v: Vec<u64> = vec![];
        for &ee in e[x].iter() {
            f(ans, e, ee);
            v.push(ans[ee]);
        }
        v.sort();
        ans[x] = v[0] + v[v.len() - 1] + 1;
    }
}

fn main() {
    let n = read::<usize>();
    let mut e = vec![Vec::new(); n + 1];
    for i in 2..n + 1 {
        let x = read::<usize>();
        e[x].push(i);
    }

    let mut ans = vec![0u64; n + 1];
    f(&mut ans, &e, 1);
    println!("{}", ans[1]);
}
