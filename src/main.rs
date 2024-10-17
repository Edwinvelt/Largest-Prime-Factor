// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?

fn main() {
    let value: u64 = 600851475143;

    let mut prime_number: u64 = 2;
    let mut highest_prime_number: u32 = 2;

    let mut used_prime_numbers: Vec<u64> = Vec::new();
    

    // while loop (if value is bigger than 1)
    while value > 1 {

        // check if number is a prime number (only dividable with it's self or 1)
        if prime_number % 2 != 1 && prime_number != 2 {
            println!("Skipped {:?} number is not odd or 2", prime_number);
            prime_number += 1;
            continue;
        }

        let result = (value as f64) % (prime_number as f64);
        // check if result has decimals (skip if it has)
        if result.fract() != 0.0 {
            println!("Result has decimals, Skipped");
            prime_number += 1;
            continue;
        }

        // divide value with biggest possible prime number
        println!("We are finding prime numbers with this one");
        println!("{:?}", result);
            // check if new value is a full number
        used_prime_numbers.push(prime_number);
        // println!("{:?}", used_prime_numbers);
        prime_number += 1;
            // update value if true, check again if false

    }
    // Print result

}
