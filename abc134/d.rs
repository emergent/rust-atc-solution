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
    let n = read::<usize>();
    let mut v = read_vec::<u32>();

    for i in (0..n / 2).rev() {
        let x = n / (i + 1);
        let mut sum_mod2 = 0;
        for j in 2..x + 1 {
            sum_mod2 = (sum_mod2 + v[(i + 1) * j - 1]) % 2;
        }
        if sum_mod2 == v[i] {
            v[i] = 0;
        } else {
            v[i] = 1;
        }
        //println!("{:?}", v);
    }

    let m = v.iter().fold(0, |sum, x| sum + x);
    println!("{}", m);
    if m != 0 {
        let balls: String = v
            .into_iter()
            .enumerate()
            .map(|(i, y)| if y == 1 { i + 1 } else { 0 })
            .filter(|&x| x != 0)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", balls);
    }
}
