fn main() {
    let sa = read::<String>().chars().collect::<Vec<_>>();
    let sb = read::<String>().chars().collect::<Vec<_>>();
    let sc = read::<String>().chars().collect::<Vec<_>>();
    let mut v = vec![sa, sb, sc];

    let mut turn = 0;
    loop {
        if v[turn].len() == 0 {
            break;
        }
        let x = v[turn].remove(0);
        match x {
            'a' => turn = 0,
            'b' => turn = 1,
            'c' => turn = 2,
            _ => panic!(),
        }
    }
    match turn {
        0 => println!("A"),
        1 => println!("B"),
        _ => println!("C"),
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
