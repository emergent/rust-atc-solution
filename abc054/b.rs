fn main() {
    let v = read_vec::<usize>();
    let n = v[0];
    let m = v[1];

    let mut im = vec![vec![false; n]; n];
    for i in 0..n {
        let line = read::<String>().chars().collect::<Vec<_>>();
        for j in 0..n {
            if line[j] == '#' {
                im[i][j] = true;
            }
        }
    }

    let mut tmp = vec![vec![false; m]; m];
    for i in 0..m {
        let line = read::<String>().chars().collect::<Vec<_>>();
        for j in 0..m {
            if line[j] == '#' {
                tmp[i][j] = true;
            }
        }
    }

    fn check(im: &Vec<Vec<bool>>, t: &Vec<Vec<bool>>, i: usize, j: usize, m: usize) -> bool {
        for x in 0..m {
            for y in 0..m {
                if t[x][y] != im[i + x][j + y] {
                    return false;
                }
            }
        }
        true
    }

    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            if check(&im, &tmp, i, j, m) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
