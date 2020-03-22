fn p(s: &String) -> bool {
    *s.chars().rev().collect::<String>() == *s
}

fn main() {
    let s = read::<String>();

    if !p(&s) {
        yn(false);
    } else {
        let slen = s.len();
        let f = s.chars().collect::<Vec<_>>()[0..((slen - 1) / 2)]
            .into_iter()
            .map(|&c| c)
            .collect::<String>();
        let l = s.chars().collect::<Vec<_>>()[((slen + 3) / 2 - 1)..slen]
            .into_iter()
            .map(|&c| c)
            .collect::<String>();
        //println!("{} {}", f, l);
        yn(p(&f) && p(&l));
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
