fn abc(v: &Vec<char>, n: usize) {
    match n {
        0 => println!("{}", v.iter().map(|&c| c).collect::<String>()),
        _ => {
            for &ch in &vec!['a', 'b', 'c'] {
                let mut va = v.clone();
                va.push(ch);
                abc(&va, n - 1);
            }
        }
    }
}

fn main() {
    let n = read::<usize>();
    let v = vec![];
    abc(&v, n);
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
