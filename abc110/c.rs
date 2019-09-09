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
    let s = read::<String>()
        .chars()
        .map(|c| c as i32 - 97)
        .collect::<Vec<i32>>();
    let t = read::<String>()
        .chars()
        .map(|c| c as i32 - 97)
        .collect::<Vec<i32>>();
    let len = s.len();

    let mut sv = vec![Vec::<i32>::new(); 26];
    let mut tv = vec![Vec::<i32>::new(); 26];
    for i in 0..len {
        sv[s[i] as usize].push(i as i32);
        tv[t[i] as usize].push(i as i32);
    }
    sv.sort();
    tv.sort();
    let mut f = true;
    for i in 0..26 {
        if sv[i] != tv[i] {
            f = false;
            break;
        }
    }
    yn(f);
}
