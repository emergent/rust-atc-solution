use std::collections::HashMap;
fn main() {
    let v = read_vec::<usize>();
    let _n = v[0];
    let m = v[1];

    let py = read_vec2::<u32>(m as u32);
    let mut py2 = py.clone();
    py2.sort_by_key(|p| p[1]);
    let mut hm = HashMap::new();
    let mut names = HashMap::new();

    for p in py2 {
        let c = hm.entry(p[0]).or_insert(0);
        *c += 1;
        names.insert((p[0],p[1]), format!("{:>06}{:>06}", p[0], *c));
    }

    for x in py {
        if let Some(t) = names.get(&(x[0], x[1])) {
            println!("{}", t);
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
