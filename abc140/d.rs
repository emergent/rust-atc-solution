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
    let v = read_vec::<usize>();
    let n = v[0];
    let k = v[1];
    let s = read::<String>().chars().collect::<Vec<char>>();

    let mut sh = vec![false; n];
    for i in 0..n {
        if (i < n - 1 && s[i] == 'R' && s[i] == s[i + 1])
            || (i > 0 && s[i] == 'L' && s[i] == s[i - 1])
        {
            sh[i] = true;
        }
    }

    let sh_count = sh.iter().filter(|&b| *b).count();
    if sh_count == n - 1 {
        println!("{}", n - 1);
        return;
    }

    let mut fushi_L = 0;
    let mut hashi_L = 0;
    let mut fushi_R = 0;
    let mut hashi_R = 0;
    for i in 0..n {
        if !sh[i] {
            if s[i] == 'L' {
                if i == 0 {
                    hashi_L = 1;
                }
                fushi_L += 1;
            } else if s[i] == 'R' {
                if i == n - 1 {
                    hashi_R = 1;
                }
                fushi_R += 1;
            }
        }
    }

    let mut sh_ans = sh_count;
    for _ in 0..k {
        if fushi_L >= 2 {
            sh_ans += 2;
            fushi_L -= 2;
        } else if fushi_R >= 2 {
            sh_ans += 2;
            fushi_R -= 2;
        } else if (hashi_R == 1 && hashi_L == 1) || (hashi_R == 0 && hashi_L == 0) {
            sh_ans += 1;
            break;
        } else if fushi_L == 1 {
            if hashi_L == 0 {
                sh_ans += 1;
            }
            fushi_L -= 1;
        } else if fushi_R == 1 {
            if hashi_R == 0 {
                sh_ans += 1;
            }
            fushi_R -= 1;
        } else {
            break;
        }
    }
    println!("{}", sh_ans);
}
