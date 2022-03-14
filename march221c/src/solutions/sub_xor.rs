use std::io;
// 4*1 = A 
// 3*2 = B 
// 2*3 = C 
// 1*4 = D 
// AAAA ABCD
//  BBB  ABC
//  BBB  BCD
//   CC   AB
//   CC   BC
//   CC   CD
//    D    A
//    D    B
//    D    C
//    D    D
//
// AAAAA
//  BBBB
//  BBBB
//   CCC
//   CCC
//   CCC
//    DD
//    DD
//    DD
//    DD
//     E
//     E
//     E
//     E
//     E

fn calc_xor(word: &str, digits: u64) -> u64 {
    let mut result = 0;
    let word_bytes = word.trim().as_bytes();
    for (i, _ch) in word_bytes.iter().enumerate() {
        let mut ones = 0;
        for j in 0..(i + 1)  {
            if word_bytes[j] == b'1' {
                ones = ones + (j + 1);
            }
        }
        if ones % 2 == 1 {
            result = result + u64::pow(2, digits as u32 - 1 - (i as u32));
        }
    }
    result
}

pub fn solution() {
    let mut test_cases = String::new();
    io::stdin().read_line(&mut test_cases).unwrap();
    let test_cases: u64 = test_cases.trim().parse().unwrap();

    for _i in 0..test_cases {
        let mut digits = String::new();
        io::stdin().read_line(&mut digits).unwrap();
        let digits: u64 = digits.trim().parse().unwrap();

        let mut binary = String::new();
        io::stdin().read_line(&mut binary).unwrap();
        println!("{}", calc_xor(&binary, digits) % 998244353);
    }
}
