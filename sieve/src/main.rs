use lazy_static::lazy_static;

use num_bigint::BigUint;
use std::sync::Mutex;



lazy_static!(    
    static ref PRIME_LIST : Mutex<Vec<BigUint>> = Mutex::new(vec![]);
    static ref ZERO : BigUint = BigUint::from(0u32);
);


//check if a number is prime
fn check_prime(ref number: &BigUint) -> bool{
    let max_check = number.sqrt();
    let prime_list_l = PRIME_LIST.lock().unwrap();
    for prime in prime_list_l.iter() {
        let remainder = *number % prime;
        if remainder == *ZERO {
            return false;
        }

        if max_check < *prime {
            break;
        }
    }
    println!("{} ", number);
    return true;
}

//write a main method that finds and prints prime numbers
fn main() {
    // let mut PRIME_LIST : Vec<&BigUint> = Vec::new();

    let THREE : BigUint = BigUint::from(3u32);
    let FIVE  : BigUint = BigUint::from(5u32);
    let SEVEN : BigUint = BigUint::from(7u32);

    PRIME_LIST.lock().unwrap().push(THREE);
    PRIME_LIST.lock().unwrap().push(FIVE);
    PRIME_LIST.lock().unwrap().push(SEVEN);

    // PRIME_LIST.push(&THREE);
    // PRIME_LIST.push(&FIVE);
    // PRIME_LIST.push(&SEVEN);

    let mut number = BigUint::from(10u32);
    loop {
        let candidate_one   : BigUint = &number + 1u32;
        let candidate_three : BigUint = &number + 3u32;
        let candidate_seven : BigUint = &number + 7u32;
        let candidate_nine  : BigUint = &number + 9u32;


        if(check_prime(&candidate_one) ){
            PRIME_LIST.lock().unwrap().push(candidate_one);
        }
        
        if(check_prime(&candidate_three) ){
            PRIME_LIST.lock().unwrap().push(candidate_three);
        }

        if(check_prime(&candidate_seven) ){
            PRIME_LIST.lock().unwrap().push(candidate_seven);
        }

        if(check_prime(&candidate_nine) ){
            PRIME_LIST.lock().unwrap().push(candidate_nine);
        }

        number += 10u32;
    }
}