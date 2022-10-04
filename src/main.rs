/// This is a simple RUST program to solve the problem 1 cited in Project Euler
/// 
/// Problem Statement#
/// 
/// Find the sum of all the multiples of 3 or 5 for the numbers less than 1000

fn main() {
    let mut multiples: Vec<u64> = Vec::new();

    for i in 0..1000 {

        if i%3 == 0 || i%5 == 0 {
            multiples.push(i);
        }

        else {
            continue;
        }
    }

    let solution: u64 = multiples.iter().sum();

    println!(" The sum of multiples of 3 or 5 less than 1000 is {}", solution);
}
