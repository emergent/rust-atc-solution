fn addll(llfrom: &Vec<i64>, llto: &mut Vec<i64>) {
    for ll in llfrom {
        let last = ll % 10;
        for i in -1..2 {
            let li = last + i;
            if 0 <= li && li <= 9 {
                llto.push(ll * 10 + li);
            }
        }
    }
}

fn main() {
    let n = read::<usize>();
    let mut lls = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut llv = lls.clone();
    loop {
        let mut llv2 = vec![];
        addll(&mut llv, &mut llv2);
        let mut llv3 = llv2.clone();
        lls.append(&mut llv3);
        llv = llv2;

        if lls.len() >= 100000 {
            break;
        }
    }
    //println!("{:?}", &lls[..100]);

    println!("{}", lls[n - 1]);
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
