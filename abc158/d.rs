fn main() {
    let mut s = read::<String>();
    let qnum = read::<u32>();

    let mut rev = false;
    let mut qs = vec![];
    for _ in 0..qnum {
        let line = read_vec::<String>();
        if line[0] == "1" {
            rev = !rev;
        }
        qs.push(line);
    }

    //if rev {
    //        s = s.chars().rev().collect::<String>();
    //    }

    rev = false;
    for q in qs {
        let t: u32 = q[0].parse().ok().unwrap();
        if t == 1 {
            rev = !rev;
        } else {
            let f: u32 = q[1].parse().ok().unwrap();
            let c: char = q[2].clone().chars().collect::<Vec<_>>()[0];
            if f == 1 {
                if !rev {
                    s.insert(0, c);
                } else {
                    s.push(c);
                }
            } else {
                if !rev {
                    s.push(c);
                } else {
                    s.insert(0, c);
                }
            }
        }
    }
    if rev {
        s = s.chars().rev().collect::<String>();
    }
    println!("{}", s);
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
