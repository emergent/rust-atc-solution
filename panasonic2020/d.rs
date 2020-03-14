fn add1(v: &mut Vec<String>, n: usize) {
    if n == 1 {
        return;
    }

    let v2 = v.clone();
    v.clear();

    for s in v2 {
        let sv = s.chars().map(|c| c as u8 - 97).collect::<Vec<_>>();
        let max = sv.iter().max().unwrap();
        //println!("{:?} {}", sv, max);
        for i in 0..max + 2 {
            let mut sv2 = sv.clone();
            sv2.push(i);
            let a = sv2.iter().map(|&i| (i + 97) as char).collect::<String>();
            v.push(a);
        }
    }

    add1(v, n - 1);
}

fn main() {
    let n = read::<usize>();
    let mut v = vec!["a".to_string()];

    add1(&mut v, n);
    v.sort();
    for s in v {
        println!("{}", s);
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
