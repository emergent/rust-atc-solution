fn is_bomb(c: char) -> bool {
    c == '#'
}

fn main() {
    let v = read_vec::<usize>();
    let h = v[0];
    let w = v[1];

    let mut vv = vec![];
    for _ in 0..h {
        vv.push(read::<String>().chars().collect::<Vec<char>>());
    }
    for i in 0..h {
        for j in 0..w {
            if vv[i][j] != '#' {
                let mut count = 0;

                if i > 0 && j > 0 && is_bomb(vv[i - 1][j - 1]) {
                    count += 1;
                }
                if i > 0 && is_bomb(vv[i - 1][j]) {
                    count += 1;
                }
                if i > 0 && j < w - 1 && is_bomb(vv[i - 1][j + 1]) {
                    count += 1;
                }
                if j < w - 1 && is_bomb(vv[i][j + 1]) {
                    count += 1;
                }
                if i < h - 1 && j < w - 1 && is_bomb(vv[i + 1][j + 1]) {
                    count += 1;
                }
                if i < h - 1 && is_bomb(vv[i + 1][j]) {
                    count += 1;
                }
                if i < h - 1 && j > 0 && is_bomb(vv[i + 1][j - 1]) {
                    count += 1;
                }
                if j > 0 && is_bomb(vv[i][j - 1]) {
                    count += 1;
                }
                vv[i][j] = count.to_string().chars().nth(0).unwrap();
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", vv[i][j]);
        }
        println!("");
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
