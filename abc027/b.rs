fn main() {
    let n = read::<usize>();
    let v = read_vec::<i32>();

    let sum = v.iter().sum::<i32>();
    if sum % n as i32 != 0 {
        println!("-1");
        return;
    }
    let ave = sum / n as i32;

    let mut f = false;
    let mut count = 0;
    let mut tmp = 0;
    let mut tmp_i = 0;
    for i in 0..n {
        if !f {
            if ave == v[i] {
                continue;
            } else {
                tmp += v[i];
                tmp_i += 1;
                f = true;
            }
        } else {
            tmp += v[i];
            tmp_i += 1;
            if tmp % tmp_i == 0 && tmp / tmp_i == ave {
                f = false;
                count += tmp_i - 1;
                tmp_i = 0;
                tmp = 0;
            }
        }
    }
    if f {
        count += tmp_i - 1;
    }
    println!("{}", count);
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
