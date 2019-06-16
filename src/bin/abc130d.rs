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
    //let n = read::<u32>();
    let v = read_vec::<u64>();
    let n = v[0] as usize;
    let k = v[1];
    let an = read_vec::<u64>();

    let mut count = 0;
    let mut sum: u64 = 0;
    let mut r: usize = 0;

    sum += an[0];
    'outer: for i in 0..n {
        loop {
            if sum >= k {
                break;
            }
            if r == n - 1 {
                break;
            } else {
                r += 1;
            }
            sum += an[r];
        }
        if sum < k {
            break;
        } else {
            //println!("{} {} {}", count , sum, r);
            count += n - r;
            sum -= an[i];
        }
    }
    println!("{}", count);
}
