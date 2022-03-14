use std::io;

pub fn solution() {
    let mut cases = String::new();
    io::stdin().read_line(&mut cases).expect("Input expected");
    let cases: i16 = cases.trim().parse().unwrap();
    for _i in 0..cases {
        let mut numbers = String::new();
        io::stdin().read_line(&mut numbers).unwrap();
        let numbers = numbers.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let answers = numbers[0];
        let marks = numbers[1];
        let max_marks = answers * 3;
        if max_marks < marks || marks == max_marks - 1 || marks == max_marks - 2 || marks == max_marks - 5 {
            println!("NO");
            continue;
        }
        let correct = if marks % 3 == 0 { marks / 3 } else { marks / 3 + 1 };
        let incorrect = if marks % 3 == 0 {
            0
        } else {
            if marks % 3 == 1 { 2 } else { 1 }
        };
        println!("YES");
        println!("{} {} {}", correct, incorrect, answers - correct - incorrect);
    }
}
