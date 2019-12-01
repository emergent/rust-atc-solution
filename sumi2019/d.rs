fn c2i(c: char) -> i32 {
    c as i32 - 48
}

fn s2i(s: String) -> i32 {
    s.parse().unwrap()
}

fn main() {
    let n = read::<usize>();
    let s = read::<String>().chars().collect::<Vec<char>>();
    let mut ans = vec![false; 1000];
    let mut d0 = vec![false; 10];
    let mut d1 = vec![false; 100];

    for i in 0..n - 2 {
        //println!("i = {}, d0 = {:?}", i, d0);
        if d0[c2i(s[i]) as usize] {
            //println!("skip d0");
            continue;
        } else {
            d0[c2i(s[i]) as usize] = true;
        }

        for j in i + 1..n - 1 {
            //println!("j = {}, d1 = {:?}", j, d1);

            let keta2 = s2i(format!("{}{}", s[i], s[j])) as usize;
            if d1[keta2] {
                //println!("skip d1");
                continue;
            } else {
                d1[keta2] = true;
            }

            for k in j + 1..n {
                let ansho = s2i(format!("{}{}{}", s[i], s[j], s[k])) as usize;
                //println!("- {}", ansho);
                ans[ansho] = true;
            }
        }
    }

    let count = ans.into_iter().filter(|&b| b == true).count();
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
