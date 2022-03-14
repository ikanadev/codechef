use std::io;

pub fn solution() {
    let mut cases = String::new();
    io::stdin().read_line(&mut cases).expect("Input expected");
    let cases: i16 = cases.trim().parse().unwrap();
    for _i in 0..cases {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Input expected");
        let n: i32 = n.trim().parse().unwrap();
        println!("{}", if n % 2 == 0 { n / 2 } else { n / 2 + 1 });
    }
}
