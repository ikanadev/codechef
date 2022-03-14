use std::io;

fn get_max(word: &str) -> i32 {
    let mut max = 0;
    let mut counter = 0;
    let word_bytes = word.trim().as_bytes();
    for (_i, ch) in word_bytes.iter().enumerate() {
        if *ch == word_bytes[0] || *ch == word_bytes[word_bytes.len() - 1] {
            max = if counter > max { counter } else { max };
            counter = 0;
            continue;
        }
        counter = counter + 1;
    }
    if max == 0 { -1 } else { max }
}

pub fn solution() {
    let mut test_cases = String::new();
    io::stdin().read_line(&mut test_cases).unwrap();
    let test_cases: i32 = test_cases.trim().parse().unwrap();

    for _i in 0..test_cases {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Expected an input");
        println!("{}", get_max(&word));
    }
}
