fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

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

fn main() {
    let v1 = read_vec::<u32>();
    let _n = v1[0];
    let m = v1[1];
    let mut min = 0;
    let mut max = 100000;

    for _i in 0..m {
        let v = read_vec::<u32>();
        let l = v[0];
        let r = v[1];

        if min < l {
            min = l;
        }
        if max > r {
            max = r;
        }
    }

    if max < min {
        println!("{}", 0);
    } else {
        println!("{}", max - min + 1);
    }
}
