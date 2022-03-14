use std::io;

fn compare(hidden: &str, guess: &str) -> String {
    let mut result = String::new();
    let guess_bytes = guess.trim().as_bytes();
    let hidden_bytes = hidden.trim().as_bytes();
    for (i, ch) in hidden_bytes.iter().enumerate() {
        if *ch == guess_bytes[i] {
            result.push('G');
        } else {
            result.push('B');
        }
    }
    result
}

pub fn solution() {
    let mut cases = String::new();
    let mut counter = 1;

    io::stdin().read_line(&mut cases).expect("Expected an input");

    let cases: i32 = cases.trim().parse().unwrap();

    while counter <= cases {
        counter = counter + 1;
        let mut hidden = String::new();
        io::stdin().read_line(&mut hidden).expect("Expected an input");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Expected an input");
        println!("{}", compare(&hidden, &guess));
    }
}
