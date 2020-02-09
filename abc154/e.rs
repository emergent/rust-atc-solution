fn next_num(n: &String, x: usize) -> (u64, u64) {
    let keta = n.len();

    for i in x..keta {
        let a = n[i..i + 1].parse::<u64>().unwrap();
        if a != 0 {
            return (a, i as u64);
        }
    }
    (0, 0)
}

fn k2(n: &String, n_first: u64) -> u64 {
    let mut ans = 0;
    let (n_second, i2) = next_num(n, 1);
    let keta = n.len() as u64;

    ans += 9 * 9 * ((keta - 2) * (keta - 2 + 1) / 2); // keta-1桁までの数
    ans += (n_first - 1) * 9 * (keta - 1); // keta桁の先頭1つ目の数字より1つ少ないケース
    if n_second > 0 {
        ans += 9 * (keta - i2 - 1); // 先頭1つ目が上限値と同じケース、かつ二桁めが1少ないケース
        ans += n_second;
    }
    ans
}

fn main() {
    let n = read::<String>();
    let k = read::<u64>();
    let keta = n.len() as u64;
    let n_first = n[0..1].parse::<u64>().unwrap();

    let mut ans = 0;
    if k == 1 {
        ans = 9 * (keta - 1) + n_first;
    } else if k == 2 {
        ans += k2(&n, n_first);
    } else if k == 3 {
        let (n_second, i2) = next_num(&n, 1);

        for i in 2..keta - 1 {
            ans += 9 * 9 * 9 * i * (i - 1) / 2;
        }
        ans += (n_first - 1) * 9 * 9 * (keta - 1) * (keta - 2) / 2;

        if n_second > 0 {
            // 先頭1桁が固定(n_first)のケース
            ans += k2(&n[i2 as usize..].to_string(), n_second);
        }
    }
    println!("{}", ans);
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
