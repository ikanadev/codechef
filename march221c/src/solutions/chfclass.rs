use std::io;

pub fn solution() {
    let mut cases = String::new();
    io::stdin().read_line(&mut cases).expect("Input expected");
    let cases: i32 = cases.trim().parse().unwrap();
    for _i in 0..cases {
        let mut days = String::new();
        io::stdin().read_line(&mut days).expect("Input expected");
        let days: i32 = days.trim().parse().unwrap();
        println!("{}", (days + 1) / 7);
    }
}
