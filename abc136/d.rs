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
fn even(x: usize) -> bool {
    x % 2 == 0
}
fn odd(x: usize) -> bool {
    x % 2 == 1
}
fn main() {
    let s1 = read::<String>();
    let s = s1.chars().collect::<Vec<char>>();
    let n = s.len();
    //println!("{:?}", s);

    let mut dp = vec![0u64; n];
    let mut next_l = 0;
    let mut last_r = n - 1;
    for i in 0..n {
        if s[i] == 'L' {
            next_l = i;
            last_r = i - 1;
            break;
        }
    }

    for i in 0..n {
        if s[i] == 'R' {
            if i > next_l {
                for j in i + 1..n {
                    if s[j] == 'L' {
                        next_l = j;
                        break;
                    }
                }
            }
            if even(next_l - i) {
                dp[next_l] += 1;
            } else {
                dp[next_l - 1] += 1;
            }
            last_r = i;
        //println!("{:?} {}", dp, first_L);
        } else {
            if even(i - last_r) {
                dp[last_r] += 1;
            } else {
                dp[last_r + 1] += 1;
            }
        }
    }
    let ans = dp
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
