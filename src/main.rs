use std::io;

fn main() {
    loop {
        println!("Print the numbers using that pattern: [N1, N2]");
        let mut input = String::new();
        println!("First number: ");
        io::stdin().read_line(&mut input).unwrap();
        let n1: i32 = input.trim().parse().unwrap();

        let mut input = String::new();
        println!("Second number: ");
        io::stdin().read_line(&mut input).unwrap();
        let n2: i32 = input.trim().parse().unwrap();

        let mut total = 0;
        for n in n1..n2 {
            for digit in n.to_string().chars() {
                total += digit.to_digit(10).unwrap() as i32;
            }
        }
        println!("Algo sum: {}", total);
        println!();
    }
}