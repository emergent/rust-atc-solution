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
    let s = read::<String>();
    let n = s.len();

    if n < 2 {
        println!("{}", 0);
        return;
    }

    let v = s.chars().collect::<Vec<char>>();
    let count0 = v.iter().filter(|&c| *c == '0').count() as i32;
    let count1 = v.iter().filter(|&c| *c == '1').count() as i32;
    let sa = (count0 - count1).abs() as usize;

    println!("{}", n - sa);
}
