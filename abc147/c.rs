fn check(a: &Vec<bool>, v: &Vec<Vec<i32>>, n: usize) -> bool {
    for i in 0..n {
        if a[i] {
            for j in 0..n {
                if v[i][j] == -1 {
                    continue;
                }

                let aj = if a[j] { 1 } else { 0 };
                if v[i][j] != aj {
                    return false;
                }
            }
        }
    }
    //println!("{:?}, {:?}", a, v);
    true
}

fn comb(vv: &mut Vec<Vec<bool>>, v: &mut Vec<bool>, n: usize) {
    if n == 0 {
        vv.push(v.clone());
    } else {
        let mut x = v.clone();
        x.push(true);
        comb(vv, &mut x, n - 1);

        let mut y = v.clone();
        y.push(false);
        comb(vv, &mut y, n - 1);
    }
}

fn combinations(n: usize) -> Vec<Vec<bool>> {
    let mut vv = vec![];
    let mut v = vec![];
    comb(&mut vv, &mut v, n);
    vv
}

fn main() {
    let n = read::<usize>();
    let mut v = vec![vec![-1; n]; n];

    for i in 0..n {
        let a = read::<usize>();
        for _ in 0..a {
            let w = read_vec::<i32>();
            v[i][(w[0] - 1) as usize] = w[1];
        }
    }

    let mut max = 0;
    let a = combinations(n);
    //println!("{:?}", a);
    for ai in a {
        if check(&ai, &v, n) {
            max = std::cmp::max(ai.iter().filter(|&t| *t).count(), max);
        }
    }
    println!("{}", max);
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
