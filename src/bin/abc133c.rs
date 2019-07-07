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
    let v = read_vec::<u64>();
    let l = v[0];
    let r = v[1];

    let mut min: u64 = 2000000000;

    for i in l..r {
        if i % 2019 == 0 {
            min = 0;
            break;
        }
        for j in i + 1..r + 1 {
            if j % 2019 == 0 {
                min = 0;
                break;
            }
            //println!("{} {} {}", i, j, i*j%2019);
            min = std::cmp::min(min, i * j % 2019);
        }

        if min == 0 {
            break;
        }
    }
    println!("{}", min);
}
