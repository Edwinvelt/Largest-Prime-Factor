// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600.851.475.143?

fn main() {
    let mut value: u64 = 600851475143;

    let mut prime_number: u64 = 2;
    // let mut highest_prime_number: u32 = 2;
    let mut factor_list: Vec<u64> = Vec::new();
  
    //  pow = prime_number * prime_number * ...
    while value > 1 && prime_number.pow(2) <= value {
        // Check if divided evenly
        if value % prime_number == 0 {
            // if evenly push to list
            factor_list.push(prime_number);
            // divide value and set it
            value /= prime_number;

            println!("{:?}", factor_list);
        }
        // This should improve performance??? not sure...
        else if prime_number > 2  {
            prime_number += 2;
        }
        else {
            prime_number += 1;
        }
    }
    if value > 1 {
        factor_list.push(value);
    }

    println!("Ended with {:?}", factor_list);

}
